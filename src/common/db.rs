use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::AppConfig;
use crate::common::error::AppError;

/// Create a database connection pool
pub async fn create_pool(config: &AppConfig) -> Result<PgPool, AppError> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .map_err(|e| AppError::Database(format!("Failed to connect to database: {}", e)))
}

/// Run database migrations using sqlx's built-in migration tracking
/// Only applies migrations that haven't been run yet (tracked in _sqlx_migrations table)
pub async fn run_migrations(pool: &PgPool) -> Result<(), AppError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to run migrations: {}", e)))?;
    
    tracing::info!("Database migrations completed");
    Ok(())
}
