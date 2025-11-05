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

/// Runtime Tokio global
static TOKIO_RUNTIME: once_cell::sync::Lazy<tokio::runtime::Runtime> =
    once_cell::sync::Lazy::new(|| {
        tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime")
    });

/// Système pour se connecter au serveur
pub async fn connect_to_server(
    address: &str,
) -> Result<Arc<Mutex<TcpStream>>, Box<dyn std::error::Error>> {
    let stream = TcpStream::connect(address).await?;
    Ok(Arc::new(Mutex::new(stream)))
}

/// Version synchrone pour le menu
pub fn connect_to_server_blocking(
    address: &str,
) -> Result<Arc<Mutex<TcpStream>>, Box<dyn std::error::Error>> {
    TOKIO_RUNTIME.block_on(connect_to_server(address))
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
    _game_state: Res<crate::game::GameState>,
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
                        TOKIO_RUNTIME.spawn(async move {
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
                        TOKIO_RUNTIME.spawn(async move {
                            if let Err(e) = send_message(&stream_clone, message).await {
                                eprintln!("Erreur envoi message: {}", e);
                            }
                        });
                    }
                    NetworkEvent::EndTurn(player_id) => {
                        let message = Message::EndTurn {
                            player_id: *player_id,
                        };
                        TOKIO_RUNTIME.spawn(async move {
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
/// Note: Ce système est appelé à chaque frame mais ne devrait lancer qu'une seule tâche
pub fn receive_from_server(
    network_connection: Option<Res<NetworkConnection>>,
    mut game_state: ResMut<crate::game::GameState>,
) {
    // Pour éviter de lancer plusieurs tâches, on vérifie si une connexion existe
    // mais on ne fait rien ici car la réception devrait être gérée différemment
    // Pour l'instant, on va simplifier en ne recevant pas de messages
    // Une meilleure implémentation utiliserait des channels ou un système d'événements
    
    if let Some(conn) = network_connection {
        if !conn.connected {
            return;
        }

        if let Some(stream) = &conn.stream {
            let stream_clone = stream.clone();
            
            // Essaie de recevoir un message de manière non-bloquante
            // Pour une implémentation simple, on pourrait utiliser un polling
            TOKIO_RUNTIME.spawn(async move {
                match receive_message(&stream_clone).await {
                    Ok(Some(message)) => {
                        match message {
                            Message::Welcome {
                                player_id,
                                world_state,
                            } => {
                                println!("✓ Bienvenue joueur {} !", player_id);
                                // TODO: Mettre à jour game_state via un canal
                            }
                            Message::Sync { world_state } => {
                                println!("✓ Synchronisation reçue");
                                // TODO: Mettre à jour game_state via un canal
                            }
                            Message::Response { success, message } => {
                                if success {
                                    println!("✓ {}", message);
                                } else {
                                    println!("✗ {}", message);
                                }
                            }
                            _ => {}
                        }
                    }
                    Ok(None) => {
                        println!("Connexion fermée par le serveur");
                    }
                    Err(e) => {
                        eprintln!("Erreur réception: {}", e);
                    }
                }
            });
        }
    }
}
