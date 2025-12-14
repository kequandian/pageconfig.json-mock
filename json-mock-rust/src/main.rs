//! JSON Mock Server - A generic JSON storage service using MongoDB
//!
//! This service provides RESTful APIs to store and retrieve arbitrary JSON data.

mod db;
mod handlers;
mod models;
mod routes;

use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::db::MongoDb;

/// Application state shared across handlers
pub struct AppState {
    pub db: MongoDb,
    pub environment: String,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Get MongoDB connection string
    let mongo_uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let db_name = std::env::var("DB_NAME")
        .unwrap_or_else(|_| "json_mock".to_string());
    let environment = std::env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_string());

    // Connect to MongoDB
    let db = MongoDb::new(&mongo_uri, &db_name)
        .await
        .expect("Failed to connect to MongoDB");

    tracing::info!("Connected to MongoDB");

    // Create application state
    let state = Arc::new(AppState { db, environment });

    // CORS configuration - allow all origins
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application routes
    let app = Router::new()
        .merge(routes::create_routes())
        .layer(cors)
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
