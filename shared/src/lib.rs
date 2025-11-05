pub mod protocol {
    use serde::{Deserialize, Serialize};

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
        pub action_points: u32,   // PA (Points d'Action)
        pub movement_points: u32, // PM (Points de Mouvement)
        pub health: u32,
        pub max_health: u32,
        pub is_alive: bool,
    }

    impl PlayerState {
        pub fn new(id: PlayerId, position: Position) -> Self {
            Self {
                id,
                position,
                action_points: 6,   // PA par défaut
                movement_points: 3, // PM par défaut
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
        Disconnect { player_id: PlayerId },
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
        EndTurn { player_id: PlayerId },
        /// Synchronisation de l'état du monde depuis le serveur
        Sync { world_state: WorldState },
        /// Message de confirmation ou d'erreur
        Response { success: bool, message: String },
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
    }
}

#[cfg(test)]
mod tests {
    use super::protocol::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_position_manhattan_distance() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(3, 4);
        assert_eq!(pos1.manhattan_distance(&pos2), 7);
    }

    #[test]
    fn test_player_state_creation() {
        let pos = Position::new(5, 5);
        let player = PlayerState::new(1, pos);
        assert_eq!(player.id, 1);
        assert_eq!(player.position, pos);
        assert_eq!(player.action_points, 6);
        assert_eq!(player.movement_points, 3);
        assert_eq!(player.health, 100);
        assert_eq!(player.max_health, 100);
        assert!(player.is_alive);
    }

    #[test]
    fn test_player_state_reset_turn() {
        let pos = Position::new(0, 0);
        let mut player = PlayerState::new(1, pos);

        // Simule l'utilisation de points
        player.action_points = 2;
        player.movement_points = 0;

        // Réinitialise le tour
        player.reset_turn();

        assert_eq!(player.action_points, 6);
        assert_eq!(player.movement_points, 3);
    }

    #[test]
    fn test_world_state_creation() {
        let world = WorldState::new(10, 10);
        assert_eq!(world.map_width, 10);
        assert_eq!(world.map_height, 10);
        assert_eq!(world.current_turn, 0);
        assert_eq!(world.turn_number, 1);
        assert!(world.players.is_empty());
    }

    #[test]
    fn test_world_state_get_player() {
        let mut world = WorldState::new(10, 10);
        let player = PlayerState::new(1, Position::new(5, 5));
        world.players.push(player.clone());

        let found = world.get_player(1);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, 1);

        let not_found = world.get_player(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_message_serialization() {
        let message = Message::Move {
            player_id: 1,
            target_position: Position::new(5, 5),
        };

        let serialized = serialization::serialize(&message).unwrap();
        assert!(!serialized.is_empty());

        let deserialized = serialization::deserialize(&serialized).unwrap();
        match deserialized {
            Message::Move {
                player_id,
                target_position,
            } => {
                assert_eq!(player_id, 1);
                assert_eq!(target_position, Position::new(5, 5));
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_welcome_message_serialization() {
        let world = WorldState::new(10, 10);
        let message = Message::Welcome {
            player_id: 1,
            world_state: world.clone(),
        };

        let serialized = serialization::serialize(&message).unwrap();
        let deserialized = serialization::deserialize(&serialized).unwrap();

        match deserialized {
            Message::Welcome {
                player_id,
                world_state,
            } => {
                assert_eq!(player_id, 1);
                assert_eq!(world_state.map_width, 10);
                assert_eq!(world_state.map_height, 10);
            }
            _ => panic!("Wrong message type"),
        }
    }
}
