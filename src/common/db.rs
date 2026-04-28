use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::fs;
use std::path::Path;

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

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), AppError> {
    // Read and execute migration files
    let migrations_dir = Path::new("migrations");
    
    if !migrations_dir.exists() {
        tracing::info!("No migrations directory found, skipping migrations");
        return Ok(());
    }
    
    // Get all SQL files sorted by name
    let mut entries: Vec<_> = fs::read_dir(migrations_dir)
        .map_err(|e| AppError::Database(format!("Failed to read migrations directory: {}", e)))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "sql").unwrap_or(false))
        .collect();
    
    entries.sort_by_key(|e| e.path());
    
    for entry in entries {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();
        
        let sql = fs::read_to_string(&path)
            .map_err(|e| AppError::Database(format!("Failed to read migration {}: {}", filename, e)))?;
        
        tracing::info!("Running migration: {}", filename);
        
        sqlx::raw_sql(&sql)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to run migration {}: {}", filename, e)))?;
    }
    
    tracing::info!("Database migrations completed");
    Ok(())
}
