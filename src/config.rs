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
    pub mollie_api_key: Option<String>,
    pub mollie_api_base_url: String,
    pub mollie_onboarding_amount_value: String,
    pub mollie_onboarding_amount_currency: String,
    pub app_public_url: String,
    pub frontend_public_url: String,
    pub keycloak_admin_client_id: Option<String>,
    pub keycloak_admin_client_secret: Option<String>,
    pub keycloak_admin_realm: Option<String>,
    pub keycloak_organization_invite_ttl_seconds: i64,
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
            .set_default("mollie_api_base_url", "https://api.mollie.com/v2")
            .expect("Failed to set default Mollie API base URL")
            .set_default("mollie_onboarding_amount_value", "29.00")
            .expect("Failed to set default Mollie onboarding amount")
            .set_default("mollie_onboarding_amount_currency", "EUR")
            .expect("Failed to set default Mollie onboarding currency")
            .set_default("app_public_url", "http://localhost:3000")
            .expect("Failed to set default app public URL")
            .set_default("frontend_public_url", "http://localhost:5173")
            .expect("Failed to set default frontend public URL")
            .set_default("keycloak_admin_realm", "")
            .expect("Failed to set default Keycloak admin realm")
            .set_default("keycloak_organization_invite_ttl_seconds", 604800)
            .expect("Failed to set default Keycloak organization invite TTL")
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
