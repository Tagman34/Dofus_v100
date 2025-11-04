use bevy::prelude::*;
use shared::protocol::{Message, PlayerId, Position};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

/// Événements réseau
#[derive(Event)]
pub enum NetworkEvent {
    SendMove(PlayerId, Position),
    SendAttack(PlayerId, PlayerId),
    EndTurn(PlayerId),
    Connected,
    Disconnected,
}

/// Ressource pour la connexion réseau
#[derive(Resource)]
pub struct NetworkConnection {
    pub stream: Option<Arc<Mutex<tokio::net::TcpStream>>>,
    pub connected: bool,
}

impl Default for NetworkConnection {
    fn default() -> Self {
        Self {
            stream: None,
            connected: false,
        }
    }
}

/// Système pour se connecter au serveur
pub async fn connect_to_server(
    address: &str,
) -> Result<Arc<Mutex<TcpStream>>, Box<dyn std::error::Error>> {
    let stream = TcpStream::connect(address).await?;
    Ok(Arc::new(Mutex::new(stream)))
}

/// Système pour envoyer un message
pub async fn send_message(
    stream: &Arc<Mutex<TcpStream>>,
    message: Message,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = shared::protocol::serialization::serialize(&message)?;
    let len = bytes.len() as u32;

    let mut stream_guard = stream.lock().await;
    stream_guard.write_u32_le(len).await?;
    stream_guard.write_all(&bytes).await?;
    Ok(())
}

/// Système pour recevoir un message
pub async fn receive_message(
    stream: &Arc<Mutex<TcpStream>>,
) -> Result<Option<Message>, Box<dyn std::error::Error>> {
    let mut stream_guard = stream.lock().await;
    let len = match stream_guard.read_u32_le().await {
        Ok(len) => len,
        Err(_) => return Ok(None),
    };

    let mut buffer = vec![0u8; len as usize];
    stream_guard.read_exact(&mut buffer).await?;

    let message = shared::protocol::serialization::deserialize(&buffer)?;
    Ok(Some(message))
}

/// Système Bevy pour gérer les événements réseau
pub fn handle_network_events(
    mut network_events: EventReader<NetworkEvent>,
    network_connection: Option<Res<NetworkConnection>>,
    mut game_state: ResMut<crate::game::GameState>,
) {
    if let Some(conn) = network_connection {
        if !conn.connected {
            return;
        }

        if let Some(stream) = &conn.stream {
            for event in network_events.read() {
                let stream_clone = stream.clone();
                match event {
                    NetworkEvent::SendMove(player_id, position) => {
                        let message = Message::Move {
                            player_id: *player_id,
                            target_position: *position,
                        };
                        tokio::spawn(async move {
                            if let Err(e) = send_message(&stream_clone, message).await {
                                eprintln!("Erreur envoi message: {}", e);
                            }
                        });
                    }
                    NetworkEvent::SendAttack(attacker_id, target_id) => {
                        let message = Message::Attack {
                            attacker_id: *attacker_id,
                            target_id: *target_id,
                        };
                        tokio::spawn(async move {
                            if let Err(e) = send_message(&stream_clone, message).await {
                                eprintln!("Erreur envoi message: {}", e);
                            }
                        });
                    }
                    NetworkEvent::EndTurn(player_id) => {
                        let message = Message::EndTurn { player_id: *player_id };
                        tokio::spawn(async move {
                            if let Err(e) = send_message(&stream_clone, message).await {
                                eprintln!("Erreur envoi message: {}", e);
                            }
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Système pour recevoir les messages du serveur
pub fn receive_from_server(
    network_connection: Option<Res<NetworkConnection>>,
    mut game_state: ResMut<crate::game::GameState>,
) {
    if let Some(conn) = network_connection {
        if !conn.connected {
            return;
        }

        if let Some(stream) = &conn.stream {
            let stream_clone = stream.clone();
            tokio::spawn(async move {
                loop {
                    match receive_message(&stream_clone).await {
                        Ok(Some(message)) => {
                            match message {
                                Message::Welcome { player_id, world_state } => {
                                    // Initialise l'état du jeu
                                    // Note: Ceci nécessiterait un système de communication
                                    // entre le runtime Tokio et Bevy, ce qui est complexe.
                                    // Pour l'instant, on laisse cette partie simplifiée.
                                    println!("Bienvenue joueur {}", player_id);
                                }
                                Message::Sync { world_state } => {
                                    // Met à jour l'état du monde
                                    // Même commentaire que ci-dessus
                                    println!("Synchronisation reçue");
                                }
                                _ => {}
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            eprintln!("Erreur réception message: {}", e);
                            break;
                        }
                    }
                }
            });
        }
    }
}

