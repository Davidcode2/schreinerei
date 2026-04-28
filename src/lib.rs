pub mod config;
pub mod common;
pub mod auth;
pub mod modules;

use sqlx::PgPool;
use crate::auth::jwks::JwksClient;
use crate::config::AppConfig;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
    pub jwks_client: JwksClient,
}
