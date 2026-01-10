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
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    if let Err(resp) = check_permission(&state) {
        return resp.into_response();
    }

    match state.form_repo.upsert(id.clone(), body.clone()).await {
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
    match query.id {
        Some(id) => {
            // Get single form by ID
            match state.form_repo.get(&id).await {
                Ok(Some(form)) => Json(ApiResponse::success(form)).into_response(),
                Ok(None) => Json(ApiResponse::error("Form not found")).into_response(),
                Err(e) => {
                    tracing::error!("Failed to get form: {}", e);
                    Json(ApiResponse::error(e.to_string())).into_response()
                }
            }
        }
        None => {
            // Get all forms
            match state.form_repo.get_all().await {
                Ok(forms) => Json(ApiResponse::success(forms)).into_response(),
                Err(e) => {
                    tracing::error!("Failed to get forms: {}", e);
                    Json(ApiResponse::error(e.to_string())).into_response()
                }
            }
        }
    }
}

/// DELETE /form/:id - Delete a form by ID
pub async fn delete_form(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Err(resp) = check_permission(&state) {
        return resp.into_response();
    }

    // Note: JsonRepository doesn't have delete, so we upsert with empty value
    // to effectively "delete" the form data. Alternatively, we could add a delete method.
    match state.form_repo.upsert(id.clone(), Value::Object(serde_json::Map::new())).await {
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
