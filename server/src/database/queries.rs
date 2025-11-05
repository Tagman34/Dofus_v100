use super::models::{Character, CharacterStats, Fight, Map, NewCharacter, NewUser, User};
use sqlx::{PgPool, Result};

/// Crée un nouvel utilisateur
#[allow(dead_code)]
pub async fn create_user(pool: &PgPool, new_user: &NewUser) -> Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password_hash, created_at, last_login, is_active
        "#,
    )
    .bind(&new_user.username)
    .bind(&new_user.email)
    .bind(&new_user.password_hash)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Récupère un utilisateur par son nom
#[allow(dead_code)]
pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, email, password_hash, created_at, last_login, is_active
        FROM users
        WHERE username = $1
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Met à jour la dernière connexion d'un utilisateur
#[allow(dead_code)]
pub async fn update_last_login(pool: &PgPool, user_id: i32) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE users
        SET last_login = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Crée un nouveau personnage
#[allow(dead_code)]
pub async fn create_character(pool: &PgPool, new_char: &NewCharacter) -> Result<Character> {
    let character = sqlx::query_as::<_, Character>(
        r#"
        INSERT INTO characters (user_id, name, position_x, position_y, map_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, name, level, experience, health, max_health, 
                  action_points, movement_points, position_x, position_y, 
                  map_id, is_alive, created_at, last_played
        "#,
    )
    .bind(new_char.user_id)
    .bind(&new_char.name)
    .bind(new_char.position_x)
    .bind(new_char.position_y)
    .bind(new_char.map_id)
    .fetch_one(pool)
    .await?;

    // Crée les stats initiales pour le personnage
    sqlx::query(
        r#"
        INSERT INTO character_stats (character_id)
        VALUES ($1)
        "#,
    )
    .bind(character.id)
    .execute(pool)
    .await?;

    Ok(character)
}

/// Récupère un personnage par son ID
#[allow(dead_code)]
pub async fn get_character_by_id(pool: &PgPool, character_id: i32) -> Result<Option<Character>> {
    let character = sqlx::query_as::<_, Character>(
        r#"
        SELECT id, user_id, name, level, experience, health, max_health,
               action_points, movement_points, position_x, position_y,
               map_id, is_alive, created_at, last_played
        FROM characters
        WHERE id = $1
        "#,
    )
    .bind(character_id)
    .fetch_optional(pool)
    .await?;

    Ok(character)
}

/// Récupère tous les personnages d'un utilisateur
#[allow(dead_code)]
pub async fn get_user_characters(pool: &PgPool, user_id: i32) -> Result<Vec<Character>> {
    let characters = sqlx::query_as::<_, Character>(
        r#"
        SELECT id, user_id, name, level, experience, health, max_health,
               action_points, movement_points, position_x, position_y,
               map_id, is_alive, created_at, last_played
        FROM characters
        WHERE user_id = $1
        ORDER BY last_played DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(characters)
}

/// Met à jour la position d'un personnage
#[allow(dead_code)]
pub async fn update_character_position(
    pool: &PgPool,
    character_id: i32,
    x: i32,
    y: i32,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE characters
        SET position_x = $1, position_y = $2, last_played = NOW()
        WHERE id = $3
        "#,
    )
    .bind(x)
    .bind(y)
    .bind(character_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Met à jour la santé d'un personnage
#[allow(dead_code)]
pub async fn update_character_health(pool: &PgPool, character_id: i32, health: i32) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE characters
        SET health = $1, is_alive = CASE WHEN $1 <= 0 THEN FALSE ELSE TRUE END
        WHERE id = $2
        "#,
    )
    .bind(health)
    .bind(character_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Récupère les stats d'un personnage
#[allow(dead_code)]
pub async fn get_character_stats(
    pool: &PgPool,
    character_id: i32,
) -> Result<Option<CharacterStats>> {
    let stats = sqlx::query_as::<_, CharacterStats>(
        r#"
        SELECT id, character_id, strength, intelligence, agility, vitality, wisdom, chance, updated_at
        FROM character_stats
        WHERE character_id = $1
        "#,
    )
    .bind(character_id)
    .fetch_optional(pool)
    .await?;

    Ok(stats)
}

/// Récupère une carte par son ID
#[allow(dead_code)]
pub async fn get_map_by_id(pool: &PgPool, map_id: i32) -> Result<Option<Map>> {
    let map = sqlx::query_as::<_, Map>(
        r#"
        SELECT id, name, width, height, map_type, difficulty_level, is_pvp_enabled, created_at
        FROM maps
        WHERE id = $1
        "#,
    )
    .bind(map_id)
    .fetch_optional(pool)
    .await?;

    Ok(map)
}

/// Récupère toutes les cartes
#[allow(dead_code)]
pub async fn get_all_maps(pool: &PgPool) -> Result<Vec<Map>> {
    let maps = sqlx::query_as::<_, Map>(
        r#"
        SELECT id, name, width, height, map_type, difficulty_level, is_pvp_enabled, created_at
        FROM maps
        ORDER BY difficulty_level, name
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(maps)
}

/// Crée un nouveau combat
#[allow(dead_code)]
pub async fn create_fight(pool: &PgPool, map_id: Option<i32>, fight_type: &str) -> Result<Fight> {
    let fight = sqlx::query_as::<_, Fight>(
        r#"
        INSERT INTO fights (map_id, fight_type)
        VALUES ($1, $2)
        RETURNING id, map_id, fight_type, current_turn, turn_number, is_active, started_at, ended_at, winner_id
        "#,
    )
    .bind(map_id)
    .bind(fight_type)
    .fetch_one(pool)
    .await?;

    Ok(fight)
}

/// Met à jour l'état d'un combat
#[allow(dead_code)]
pub async fn update_fight_status(
    pool: &PgPool,
    fight_id: i32,
    is_active: bool,
    winner_id: Option<i32>,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE fights
        SET is_active = $1, winner_id = $2, ended_at = CASE WHEN $1 = FALSE THEN NOW() ELSE ended_at END
        WHERE id = $3
        "#,
    )
    .bind(is_active)
    .bind(winner_id)
    .bind(fight_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Réinitialise les PA/PM d'un personnage
#[allow(dead_code)]
pub async fn reset_character_turn_points(pool: &PgPool, character_id: i32) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE characters
        SET action_points = 6, movement_points = 3
        WHERE id = $1
        "#,
    )
    .bind(character_id)
    .execute(pool)
    .await?;

    Ok(())
}
