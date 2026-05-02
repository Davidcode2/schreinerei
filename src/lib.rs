pub mod auth;
pub mod common;
pub mod config;
pub mod modules;

use crate::auth::jwks::JwksClient;
use crate::config::AppConfig;
use sqlx::PgPool;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
    pub jwks_client: JwksClient,
}
