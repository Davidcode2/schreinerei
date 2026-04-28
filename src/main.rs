use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
    middleware,
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use schreinerei::{
    auth::jwks::JwksClient,
    auth::middleware::{AuthState, auth_middleware},
    common::db::{create_pool, run_migrations},
    config::AppConfig,
    modules::iam::api::routes::create_router as iam_router,
    AppState,
};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "schreinerei=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::load();
    tracing::info!("Loaded configuration for server at {}:{}", config.host, config.port);

    // Initialize database pool
    let pool = create_pool(&config).await
        .expect("Failed to create database pool");
    tracing::info!("Database pool created");

    // Run migrations (only applies unapplied migrations, tracked in _sqlx_migrations)
    run_migrations(&pool).await
        .expect("Failed to run migrations");

    // Initialize JWKS client
    let jwks_client = JwksClient::new(&config.keycloak_url, &config.keycloak_realm);
    
    // Fetch initial JWKS
    jwks_client.fetch_jwks().await
        .expect("Failed to fetch initial JWKS");
    
    // Start background refresh
    jwks_client.clone().refresh_periodically();

    // Create app state
    let state = AppState {
        config: config.clone(),
        pool,
        jwks_client: jwks_client.clone(),
    };

    // Auth state for middleware
    let auth_state = AuthState {
        jwks_client,
        jwt_issuer: config.jwt_issuer.clone(),
    };

    // Build router with health endpoint and IAM routes
    let app = Router::new()
        .route("/health", get(health_handler))
        .merge(iam_router())
        .layer(middleware::from_fn_with_state(auth_state, auth_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Invalid address");

    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "schreinerei"
    })))
}
