<<<<<<< HEAD
mod game;
mod handler;
mod session;

use crate::game::Game;
use crate::handler::broadcast_world_state;
use crate::session::handle_client;
use shared::protocol::{Message, PlayerId, Position};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

const DEFAULT_PORT: u16 = 8080;
const MAP_WIDTH: i32 = 10;
const MAP_HEIGHT: i32 = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Serveur Dofus-like démarré sur {}", addr);

    let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game::new(MAP_WIDTH, MAP_HEIGHT)));
    let (broadcast_tx, mut broadcast_rx) = tokio::sync::mpsc::unbounded_channel::<(PlayerId, Message)>();

    // Tâche pour gérer les broadcasts
    let broadcast_tx_clone = broadcast_tx.clone();
    let game_clone = game.clone();
    tokio::spawn(async move {
        while let Some((player_id, message)) = broadcast_rx.recv().await {
            // Traite les messages de broadcast
            if let Message::Disconnect { player_id: _ } = message {
                // Gère la déconnexion
            }
            // Broadcast l'état du monde après chaque action
            broadcast_world_state(game_clone.clone(), broadcast_tx_clone.clone()).await;
        }
    });

    loop {
        tokio::select! {
            // Accepte de nouvelles connexions
            Ok((stream, addr)) = listener.accept() => {
                println!("Nouvelle connexion depuis {}", addr);
                let game_clone = game.clone();
                let broadcast_tx_clone = broadcast_tx.clone();

                tokio::spawn(async move {
                    // Ajoute un nouveau joueur au jeu
                    let player_id = {
                        let mut game_guard = game_clone.lock().await;
                        // Position initiale aléatoire
                        let x = fastrand::i32(0..MAP_WIDTH);
                        let y = fastrand::i32(0..MAP_HEIGHT);
                        game_guard.add_player(Position::new(x, y))
                    };

                    println!("Joueur {} connecté", player_id);

                    // Gère la connexion client
                    if let Err(e) = handle_client(stream, player_id, game_clone.clone(), broadcast_tx_clone.clone()).await {
                        eprintln!("Erreur lors de la gestion du client {}: {}", player_id, e);
                    }

                    // Retire le joueur du jeu
                    {
                        let mut game_guard = game_clone.lock().await;
                        game_guard.remove_player(player_id);
                    }
                    println!("Joueur {} déconnecté", player_id);
                });
            }
        }
    }
=======
fn main() {
    println!("Hello, world!");
>>>>>>> origin/main
}
