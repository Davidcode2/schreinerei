use axum::Router;

use crate::AppState;

/// Create the inventory API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Routes will be added in Task 5
}
