use shared::protocol::{Message, PlayerId};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Gestionnaire de session pour un client connecté
#[allow(dead_code)]
pub struct Session {
    pub player_id: PlayerId,
    pub sender: mpsc::UnboundedSender<Message>,
}

#[allow(dead_code)]
impl Session {
    pub fn new(player_id: PlayerId) -> (Self, mpsc::UnboundedReceiver<Message>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (
            Self {
                player_id,
                sender: tx,
            },
            rx,
        )
    }

    /// Envoie un message au client
    pub fn send(&self, message: Message) -> Result<(), String> {
        self.sender
            .send(message)
            .map_err(|_| "Impossible d'envoyer le message".to_string())
    }
}

/// Gère la communication TCP avec un client
pub async fn handle_client(
    stream: tokio::net::TcpStream,
    player_id: PlayerId,
    game: Arc<tokio::sync::Mutex<crate::game::Game>>,
    broadcast_tx: mpsc::UnboundedSender<(PlayerId, Message)>,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::handler::handle_message;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    // Envoie un message de bienvenue
    let world_state = {
        let game_guard = game.lock().await;
        game_guard.get_world_state_clone()
    };

    let welcome = Message::Welcome {
        player_id,
        world_state,
    };

    let welcome_bytes = shared::protocol::serialization::serialize(&welcome)?;
    let len = welcome_bytes.len() as u32;

    let (mut read_stream, mut write_stream) = stream.into_split();

    // Envoie le message de bienvenue
    write_stream.write_u32_le(len).await?;
    write_stream.write_all(&welcome_bytes).await?;

    // Canal pour recevoir les messages à envoyer au client
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // Tâche pour envoyer les messages au client
    let write_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Ok(bytes) = shared::protocol::serialization::serialize(&message) {
                let len = bytes.len() as u32;
                if write_stream.write_u32_le(len).await.is_ok() {
                    if write_stream.write_all(&bytes).await.is_err() {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    });

    // Tâche pour lire les messages du client
    let game_for_read = game.clone();
    let broadcast_tx_for_read = broadcast_tx.clone();
    let tx_for_read = tx.clone();
    let read_task = tokio::spawn(async move {
        let mut buffer = vec![0u8; 4096];
        loop {
            match read_stream.read_u32_le().await {
                Ok(len) => {
                    if len as usize > buffer.len() {
                        buffer.resize(len as usize, 0);
                    }
                    if read_stream
                        .read_exact(&mut buffer[..len as usize])
                        .await
                        .is_ok()
                    {
                        if let Ok(message) =
                            shared::protocol::serialization::deserialize(&buffer[..len as usize])
                        {
                            // Traite le message
                            if let Ok(Some(response)) =
                                handle_message(message.clone(), player_id, game_for_read.clone())
                                    .await
                            {
                                let _ = tx_for_read.send(response);
                            }
                            // Broadcast le message pour synchronisation
                            let _ = broadcast_tx_for_read.send((player_id, message));
                        }
                    } else {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // Attend la fin des tâches
    tokio::select! {
        _ = read_task => {},
        _ = write_task => {},
    }

    // Envoie un message de déconnexion
    let _ = broadcast_tx.send((player_id, Message::Disconnect { player_id }));

    Ok(())
}
