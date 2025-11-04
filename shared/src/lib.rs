<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> df607ba4828db16e8f21f106df1e1970d7dff721
pub mod protocol {
    use serde::{Serialize, Deserialize};

    /// Identifiant unique d'un joueur
    pub type PlayerId = u32;

    /// Position sur la grille (x, y)
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    impl Position {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        /// Calcule la distance de Manhattan entre deux positions
        pub fn manhattan_distance(&self, other: &Position) -> i32 {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }
    }

    /// État d'un joueur dans le jeu
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PlayerState {
        pub id: PlayerId,
        pub position: Position,
        pub action_points: u32,      // PA (Points d'Action)
        pub movement_points: u32,     // PM (Points de Mouvement)
        pub health: u32,
        pub max_health: u32,
        pub is_alive: bool,
    }

    impl PlayerState {
        pub fn new(id: PlayerId, position: Position) -> Self {
            Self {
                id,
                position,
                action_points: 6,     // PA par défaut
                movement_points: 3,    // PM par défaut
                health: 100,
                max_health: 100,
                is_alive: true,
            }
        }

        /// Réinitialise les PA et PM au début d'un tour
        pub fn reset_turn(&mut self) {
            self.action_points = 6;
            self.movement_points = 3;
        }
    }

    /// État du monde de jeu
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct WorldState {
        pub players: Vec<PlayerState>,
        pub current_turn: PlayerId,
        pub turn_number: u32,
        pub map_width: i32,
        pub map_height: i32,
    }

    impl WorldState {
        pub fn new(map_width: i32, map_height: i32) -> Self {
            Self {
                players: Vec::new(),
                current_turn: 0,
                turn_number: 1,
                map_width,
                map_height,
            }
        }

        /// Trouve un joueur par son ID
        pub fn get_player(&self, player_id: PlayerId) -> Option<&PlayerState> {
            self.players.iter().find(|p| p.id == player_id)
        }

        /// Trouve un joueur mutable par son ID
        pub fn get_player_mut(&mut self, player_id: PlayerId) -> Option<&mut PlayerState> {
            self.players.iter_mut().find(|p| p.id == player_id)
        }
    }

    /// Messages réseau échangés entre client et serveur
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum Message {
        /// Connexion d'un nouveau joueur
        Connect {
            player_id: PlayerId,
            player_name: String,
        },
        /// Déconnexion d'un joueur
        Disconnect {
            player_id: PlayerId,
        },
        /// Déplacement d'un joueur
        Move {
            player_id: PlayerId,
            target_position: Position,
        },
        /// Attaque d'un joueur sur un autre
        Attack {
            attacker_id: PlayerId,
            target_id: PlayerId,
        },
        /// Fin du tour d'un joueur
        EndTurn {
            player_id: PlayerId,
        },
        /// Synchronisation de l'état du monde depuis le serveur
        Sync {
            world_state: WorldState,
        },
        /// Message de confirmation ou d'erreur
        Response {
            success: bool,
            message: String,
        },
        /// Message de bienvenue avec état initial
        Welcome {
            player_id: PlayerId,
            world_state: WorldState,
        },
    }

    /// Fonctions utilitaires pour la sérialisation/désérialisation
    pub mod serialization {
        use super::Message;
        use bincode;

        /// Sérialise un message en bytes
        pub fn serialize(message: &Message) -> Result<Vec<u8>, bincode::Error> {
            bincode::serialize(message)
        }

        /// Désérialise un message depuis des bytes
        pub fn deserialize(bytes: &[u8]) -> Result<Message, bincode::Error> {
            bincode::deserialize(bytes)
        }
<<<<<<< HEAD
=======
=======
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
>>>>>>> origin/main
>>>>>>> df607ba4828db16e8f21f106df1e1970d7dff721
    }
}
