use crate::game::Game;
use shared::protocol::{Message, PlayerId, Position};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Gère les messages reçus des clients
pub async fn handle_message(
    message: Message,
    player_id: PlayerId,
    game: Arc<Mutex<Game>>,
) -> Result<Option<Message>, String> {
    match message {
        Message::Move { player_id: msg_player_id, target_position } => {
            if msg_player_id != player_id {
                return Err("ID joueur incorrect".to_string());
            }

            let mut game_guard = game.lock().await;
            match game_guard.move_player(player_id, target_position) {
                Ok(()) => Ok(Some(Message::Response {
                    success: true,
                    message: "Déplacement réussi".to_string(),
                })),
                Err(e) => Ok(Some(Message::Response {
                    success: false,
                    message: e,
                })),
            }
        }

        Message::Attack { attacker_id, target_id } => {
            if attacker_id != player_id {
                return Err("ID attaquant incorrect".to_string());
            }

            let mut game_guard = game.lock().await;
            match game_guard.attack(attacker_id, target_id) {
                Ok(damage) => Ok(Some(Message::Response {
                    success: true,
                    message: format!("Attaque réussie ! {} dégâts infligés", damage),
                })),
                Err(e) => Ok(Some(Message::Response {
                    success: false,
                    message: e,
                })),
            }
        }

        Message::EndTurn { player_id: msg_player_id } => {
            if msg_player_id != player_id {
                return Err("ID joueur incorrect".to_string());
            }

            let mut game_guard = game.lock().await;
            match game_guard.end_turn(player_id) {
                Ok(()) => Ok(Some(Message::Response {
                    success: true,
                    message: "Tour terminé".to_string(),
                })),
                Err(e) => Ok(Some(Message::Response {
                    success: false,
                    message: e,
                })),
            }
        }

        Message::Connect { player_id: _, player_name: _ } => {
            // La connexion est gérée dans main.rs
            Ok(None)
        }

        Message::Disconnect { player_id: msg_player_id } => {
            if msg_player_id == player_id {
                let mut game_guard = game.lock().await;
                game_guard.remove_player(player_id);
            }
            Ok(None)
        }

        _ => Err("Message non géré".to_string()),
    }
}

/// Broadcast l'état du monde à tous les clients
pub async fn broadcast_world_state(
    game: Arc<Mutex<Game>>,
    broadcast_tx: tokio::sync::mpsc::UnboundedSender<(PlayerId, Message)>,
) {
    let world_state = {
        let game_guard = game.lock().await;
        game_guard.get_world_state_clone()
    };

    let sync_message = Message::Sync {
        world_state: world_state.clone(),
    };

    // Envoie le message de synchronisation à tous les joueurs
    for player in &world_state.players {
        let _ = broadcast_tx.send((player.id, sync_message.clone()));
    }
}

