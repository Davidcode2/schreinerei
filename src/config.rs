use config::Config;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub jwt_issuer: String,
    pub run_migrations: bool,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Self {
        // Load .env file if present
        dotenvy::dotenv().ok();

        let config = Config::builder()
            .set_default("host", "0.0.0.0")
            .expect("Failed to set default host")
            .set_default("port", "3000")
            .expect("Failed to set default port")
            .set_default("run_migrations", true)
            .expect("Failed to set default run_migrations")
            .add_source(config::Environment::default())
            .build()
            .expect("Failed to build configuration");

        let app_config: AppConfig = config
            .try_deserialize()
            .expect("Failed to deserialize configuration");

        // Validate required fields
        if app_config.database_url.is_empty() {
            panic!("DATABASE_URL is required");
        }
        if app_config.keycloak_url.is_empty() {
            panic!("KEYCLOAK_URL is required");
        }
        if app_config.keycloak_realm.is_empty() {
            panic!("KEYCLOAK_REALM is required");
        }
        if app_config.jwt_issuer.is_empty() {
            panic!("JWT_ISSUER is required");
        }

        app_config
    }
}
