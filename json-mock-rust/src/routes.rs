//! Route configuration

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::AppState;

/// Create all application routes
pub fn create_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        // Form routes
        .route("/forms", get(handlers::get_forms))
        .route("/form", get(handlers::get_forms))
        .route("/form/:id", post(handlers::upsert_form))
        .route("/form/:id", delete(handlers::delete_form))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routes_creation() {
        // Just verify routes can be created without panic
        let _routes: Router<Arc<AppState>> = create_routes();
    }
}
