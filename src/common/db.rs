use std::path::Path;

use sqlx::migrate::{Migrate, MigrateError, Migrator};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::common::error::AppError;
use crate::config::AppConfig;

const DOCUMENT_ATTACHMENT_MIGRATION_VERSION: i64 = 15;

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
    let migrator = load_migrator().await?;
    let mut conn = pool.acquire().await.map_err(database_error)?;
    conn.lock().await.map_err(migration_error)?;

    let result = match unlocked_migrator(&migrator).run_direct(&mut *conn).await {
        Ok(()) => Ok(()),
        Err(MigrateError::VersionMismatch(DOCUMENT_ATTACHMENT_MIGRATION_VERSION)) => {
            repair_document_attachment_migration(&migrator, &mut conn).await
        }
        Err(e) => Err(migration_error(e)),
    };

    let unlock_result = conn.unlock().await.map_err(migration_error);
    result?;
    unlock_result?;

    tracing::info!("Database migrations completed");
    Ok(())
}

async fn repair_document_attachment_migration(
    migrator: &Migrator,
    conn: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> Result<(), AppError> {
    ensure_document_attachment_schema_matches(conn).await?;
    update_document_attachment_checksum(migrator, conn).await?;
    unlocked_migrator(migrator)
        .run_direct(&mut **conn)
        .await
        .map_err(migration_error)?;

    tracing::warn!(
        migration = DOCUMENT_ATTACHMENT_MIGRATION_VERSION,
        "Repaired SQLx checksum mismatch for previously applied migration"
    );

    Ok(())
}

async fn ensure_document_attachment_schema_matches(
    conn: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> Result<(), AppError> {
    let has_original_filename: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM information_schema.columns
            WHERE table_schema = current_schema()
              AND table_name = 'site_activity_attachments'
              AND column_name = 'original_filename'
        )
        "#,
    )
    .fetch_one(&mut **conn)
    .await
    .map_err(database_error)?;

    let still_has_activity_unique: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM pg_constraint c
            JOIN pg_class t ON t.oid = c.conrelid
            JOIN pg_namespace n ON n.oid = t.relnamespace
            WHERE n.nspname = current_schema()
              AND t.relname = 'site_activity_attachments'
              AND c.contype = 'u'
              AND pg_get_constraintdef(c.oid) ILIKE '%(tenant_id, activity_id)%'
        )
        "#,
    )
    .fetch_one(&mut **conn)
    .await
    .map_err(database_error)?;

    if has_original_filename && !still_has_activity_unique {
        return Ok(());
    }

    Err(AppError::Database(
        "Failed to run migrations: migration 15 checksum mismatch cannot be auto-repaired because the live schema does not match the expected post-migration state".to_string(),
    ))
}

async fn update_document_attachment_checksum(
    migrator: &Migrator,
    conn: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> Result<(), AppError> {
    let checksum = migrator
        .iter()
        .find(|migration| migration.version == DOCUMENT_ATTACHMENT_MIGRATION_VERSION)
        .ok_or_else(|| {
            AppError::Database("Failed to find migration 15 in the compiled migrator".to_string())
        })?
        .checksum
        .to_vec();

    let updated_rows = sqlx::query(
        r#"
        UPDATE _sqlx_migrations
        SET checksum = $1
        WHERE version = $2
          AND success = true
        "#,
    )
    .bind(checksum)
    .bind(DOCUMENT_ATTACHMENT_MIGRATION_VERSION)
    .execute(&mut **conn)
    .await
    .map_err(database_error)?
    .rows_affected();

    if updated_rows == 1 {
        return Ok(());
    }

    Err(AppError::Database(
        "Failed to repair migration 15 checksum because no successful applied migration row was updated".to_string(),
    ))
}

async fn load_migrator() -> Result<Migrator, AppError> {
    Migrator::new(Path::new("./migrations"))
        .await
        .map_err(migration_error)
}

fn unlocked_migrator(migrator: &Migrator) -> Migrator {
    Migrator {
        migrations: migrator.migrations.clone(),
        ignore_missing: migrator.ignore_missing,
        locking: false,
        no_tx: migrator.no_tx,
    }
}

fn migration_error(error: MigrateError) -> AppError {
    AppError::Database(format!("Failed to run migrations: {error}"))
}

fn database_error(error: sqlx::Error) -> AppError {
    AppError::Database(format!("Failed to run migrations: {error}"))
}
