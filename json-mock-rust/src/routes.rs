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
        // Posts routes
        .route("/posts", post(handlers::create_post))
        .route("/posts", put(handlers::update_post))
        .route("/posts/:id", get(handlers::get_post_by_id))
        .route("/posts/:id", delete(handlers::delete_post))
        // Form routes
        .route("/form", get(handlers::get_forms))
        .route("/form/:id", post(handlers::upsert_form))
        .route("/form/:id", delete(handlers::delete_form))
        // Data routes (must be before generic /:name)
        .route("/data", post(handlers::post_data_batch))
        .route("/data/:name", get(handlers::get_data_collection))
        .route("/data/:name", post(handlers::post_data_collection))
        .route("/data/:name", delete(handlers::delete_data_collection))
        // Generic route (catch-all for any collection name)
        .route("/:name", get(handlers::get_by_name))
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
