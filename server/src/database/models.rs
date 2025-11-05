use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Modèle pour un utilisateur
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

/// Modèle pour un personnage
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub level: i32,
    pub experience: i64,
    pub health: i32,
    pub max_health: i32,
    pub action_points: i32,
    pub movement_points: i32,
    pub position_x: i32,
    pub position_y: i32,
    pub map_id: Option<i32>,
    pub is_alive: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_played: chrono::DateTime<chrono::Utc>,
}

/// Modèle pour une carte/map
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Map {
    pub id: i32,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub map_type: String,
    pub difficulty_level: i32,
    pub is_pvp_enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Modèle pour un combat
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Fight {
    pub id: i32,
    pub map_id: Option<i32>,
    pub fight_type: String,
    pub current_turn: i32,
    pub turn_number: i32,
    pub is_active: bool,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub winner_id: Option<i32>,
}

/// Modèle pour un participant d'un combat
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FightParticipant {
    pub id: i32,
    pub fight_id: i32,
    pub character_id: i32,
    pub team: i32,
    pub is_alive: bool,
    pub damage_dealt: i32,
    pub damage_taken: i32,
    pub turns_played: i32,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub left_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Modèle pour les statistiques d'un personnage
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterStats {
    pub id: i32,
    pub character_id: i32,
    pub strength: i32,
    pub intelligence: i32,
    pub agility: i32,
    pub vitality: i32,
    pub wisdom: i32,
    pub chance: i32,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Données pour créer un nouveau personnage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCharacter {
    pub user_id: i32,
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub map_id: Option<i32>,
}

/// Données pour créer un nouvel utilisateur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
