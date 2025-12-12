//! HTTP request handlers

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::sync::Arc;

use crate::models::{ApiResponse, IdQuery};
use crate::AppState;

/// Check if updates are allowed (not in production)
fn check_permission(state: &AppState) -> Result<(), (StatusCode, Json<ApiResponse<Value>>)> {
    if state.environment == "production" {
        Err((
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error(
                "update not allowed in production environment!",
            )),
        ))
    } else {
        Ok(())
    }
}

// ==================== Form Handlers ====================

/// POST /form/:id - Create or update a form
pub async fn upsert_form(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    if let Err(resp) = check_permission(&state) {
        return resp.into_response();
    }

    match state.db.upsert_form(id, body.clone()).await {
        Ok(_) => Json(ApiResponse::success(body)).into_response(),
        Err(e) => {
            tracing::error!("Failed to upsert form: {}", e);
            Json(ApiResponse::error(e.to_string())).into_response()
        }
    }
}

/// GET /form - Get forms (optionally by ID)
pub async fn get_forms(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdQuery>,
) -> impl IntoResponse {
    let id = query.id.and_then(|s| s.parse::<i64>().ok());

    match state.db.get_forms(id).await {
        Ok(forms) => Json(ApiResponse::success(forms)),
        Err(e) => {
            tracing::error!("Failed to get forms: {}", e);
            Json(ApiResponse::error(e.to_string()))
        }
    }
}

/// DELETE /form/:id - Delete a form by ID
pub async fn delete_form(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    if let Err(resp) = check_permission(&state) {
        return resp.into_response();
    }

    match state.db.delete_form(id).await {
        Ok(_) => Json(ApiResponse::success(Value::Array(vec![]))).into_response(),
        Err(e) => {
            tracing::error!("Failed to delete form: {}", e);
            Json(ApiResponse::error(e.to_string())).into_response()
        }
    }
}

// ==================== Health Check ====================

/// GET /health - Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
