pub mod models;
pub mod queries;

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Result};
use std::time::Duration;

/// Configuration de la base de données
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub connection_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/dofus_game".to_string()),
            max_connections: 5,
            connection_timeout: Duration::from_secs(30),
        }
    }
}

/// Crée un pool de connexions à la base de données
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.connection_timeout)
        .connect(&config.database_url)
        .await?;

    Ok(pool)
}

/// Exécute les migrations de la base de données
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    // Lit le fichier de migration
    let migration_sql = include_str!("../../migrations/001_init.sql");

    // Exécute la migration
    sqlx::query(migration_sql).execute(pool).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Ignorer par défaut car nécessite une base de données
    async fn test_database_connection() {
        let config = DatabaseConfig::default();
        let pool = create_pool(&config).await;
        assert!(pool.is_ok());
    }
}
