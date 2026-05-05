use axum::{http::StatusCode, middleware, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use schreinerei::{
    auth::jwks::JwksClient,
    auth::middleware::{auth_middleware, AuthState},
    common::db::{create_pool, run_migrations},
    config::AppConfig,
    modules::fleet::api::routes::create_router as fleet_router,
    modules::iam::api::routes::create_router as iam_router,
    modules::inventory::api::routes::create_router as inventory_router,
    modules::projects::api::routes::create_router as projects_router,
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

    let run_mode = std::env::args().nth(1);

    // Load configuration
    let config = AppConfig::load();
    tracing::info!(
        "Loaded configuration for server at {}:{}",
        config.host,
        config.port
    );

    // Initialize database pool
    let pool = create_pool(&config)
        .await
        .expect("Failed to create database pool");
    tracing::info!("Database pool created");

    if run_mode.as_deref() == Some("migrate") {
        run_migrations(&pool)
            .await
            .expect("Failed to run migrations");
        tracing::info!("Migration command completed successfully");
        return;
    }

    if config.run_migrations {
        run_migrations(&pool)
            .await
            .expect("Failed to run migrations");
    }

    // Initialize JWKS client
    let jwks_client = JwksClient::new(&config.keycloak_url, &config.keycloak_realm);

    // Fetch initial JWKS
    jwks_client
        .fetch_jwks()
        .await
        .expect("Failed to fetch initial JWKS");

    // Start background refresh
    jwks_client.clone().refresh_periodically();

    // Create app state
    let state = AppState {
        config: config.clone(),
        pool: pool.clone(),
        jwks_client: jwks_client.clone(),
    };

    // Auth state for middleware
    let auth_state = AuthState {
        jwks_client,
        jwt_issuer: config.jwt_issuer.clone(),
        pool,
    };

    // Build router with health endpoint and IAM routes
    let app = Router::new()
        .route("/health", get(health_handler))
        .merge(iam_router())
        .merge(inventory_router())
        .merge(projects_router())
        .merge(fleet_router())
        .layer(middleware::from_fn_with_state(auth_state, auth_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Invalid address");

    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::Json(serde_json::json!({
            "status": "healthy",
            "service": "schreinerei"
        })),
    )
}
