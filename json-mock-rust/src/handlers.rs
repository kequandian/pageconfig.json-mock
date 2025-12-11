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
fn check_permission(state: &AppState) -> Result<(), Json<ApiResponse<Value>>> {
    if state.environment == "production" {
        Err(Json(ApiResponse::error(
            "update not allowed in production environment!",
        )))
    } else {
        Ok(())
    }
}

// ==================== Posts Handlers ====================

/// GET /posts/:id - Get a post by ID
pub async fn get_post_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.db.get_post(&id).await {
        Ok(Some(post)) => Json(post),
        Ok(None) => Json(Value::Null),
        Err(e) => {
            tracing::error!("Failed to get post: {}", e);
            Json(Value::Null)
        }
    }
}

/// POST /posts - Create a new post
pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    if let Err(resp) = check_permission(&state) {
        return resp;
    }

    match state.db.push_post(body).await {
        Ok(post) => Json(post),
        Err(e) => {
            tracing::error!("Failed to create post: {}", e);
            Json(Value::Null)
        }
    }
}

/// PUT /posts - Update a post
pub async fn update_post(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    if let Err(resp) = check_permission(&state) {
        return resp;
    }

    match state.db.update_post(body).await {
        Ok(Some(post)) => Json(post),
        Ok(None) => Json(Value::Null),
        Err(e) => {
            tracing::error!("Failed to update post: {}", e);
            Json(Value::Null)
        }
    }
}

/// DELETE /posts/:id - Delete a post by ID
pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Err(resp) = check_permission(&state) {
        return resp;
    }

    match state.db.delete_post(&id).await {
        Ok(deleted) => {
            if deleted {
                Json(serde_json::json!([{"id": id}]))
            } else {
                Json(serde_json::json!([]))
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete post: {}", e);
            Json(serde_json::json!([]))
        }
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
        return resp;
    }

    match state.db.upsert_form(id, body.clone()).await {
        Ok(_) => Json(ApiResponse::success(body)),
        Err(e) => {
            tracing::error!("Failed to upsert form: {}", e);
            Json(ApiResponse::error(e.to_string()))
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
        return resp;
    }

    match state.db.delete_form(id).await {
        Ok(_) => Json(serde_json::json!([])),
        Err(e) => {
            tracing::error!("Failed to delete form: {}", e);
            Json(ApiResponse::error(e.to_string()))
        }
    }
}

// ==================== Data Handlers ====================

/// POST /data - Batch update root-level keys
pub async fn post_data_batch(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    if let Err(resp) = check_permission(&state) {
        return resp;
    }

    if let Some(obj) = body.as_object() {
        for (key, value) in obj {
            if !key.is_empty() && !value.is_null() {
                if let Err(e) = state.db.set_global(key, value.clone()).await {
                    tracing::error!("Failed to set global key {}: {}", key, e);
                }
            }
        }
    }

    Json(ApiResponse::success(body))
}

/// POST /data/:name - Insert into a named collection
pub async fn post_data_collection(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    if let Err(resp) = check_permission(&state) {
        return resp;
    }

    // Check if ID exists
    if let Some(id) = body.get("id").and_then(|v| v.as_i64()) {
        match state.db.exists_by_id(&name, id).await {
            Ok(true) => {
                return Json(ApiResponse::error("data conflict!"));
            }
            Err(e) => {
                tracing::error!("Failed to check existence: {}", e);
            }
            _ => {}
        }
    }

    match state.db.insert(&name, body.clone()).await {
        Ok(_) => Json(ApiResponse::success(body)),
        Err(e) => {
            tracing::error!("Failed to insert data: {}", e);
            Json(ApiResponse::error(e.to_string()))
        }
    }
}

/// GET /data/:name - Get data from a named collection
pub async fn get_data_collection(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(query): Query<IdQuery>,
) -> impl IntoResponse {
    let result = if let Some(id_str) = query.id {
        if let Ok(id) = id_str.parse::<i64>() {
            state.db.get_by_id(&name, id).await.map(|opt| opt.unwrap_or(Value::Null))
        } else {
            Ok(Value::Null)
        }
    } else {
        state.db.get_all(&name).await.map(Value::Array)
    };

    match result {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => {
            tracing::error!("Failed to get data: {}", e);
            Json(ApiResponse::error(e.to_string()))
        }
    }
}

/// DELETE /data/:name - Delete from a named collection
pub async fn delete_data_collection(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(query): Query<IdQuery>,
) -> impl IntoResponse {
    let result = if let Some(id) = query.id.clone() {
        state.db.delete_by_id(&name, &id).await
    } else {
        state.db.delete_all(&name).await
    };

    match result {
        Ok(_) => Json(ApiResponse::success(query.id.unwrap_or_default())),
        Err(e) => {
            tracing::error!("Failed to delete data: {}", e);
            Json(ApiResponse::error(e.to_string()))
        }
    }
}

// ==================== Generic Handlers ====================

/// GET /:name - Get any collection or global value by name
pub async fn get_by_name(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(query): Query<IdQuery>,
) -> impl IntoResponse {
    // First try as a collection
    let collection_result = if let Some(id_str) = &query.id {
        if let Ok(id) = id_str.parse::<i64>() {
            state.db.get_by_id(&name, id).await
        } else {
            Ok(None)
        }
    } else {
        state.db.get_all(&name).await.map(|v| {
            if v.is_empty() {
                None
            } else {
                Some(Value::Array(v))
            }
        })
    };

    match collection_result {
        Ok(Some(data)) => return Json(ApiResponse::success(data)),
        Ok(None) => {
            // Try global store if collection is empty
            if let Ok(Some(global_data)) = state.db.get_global(&name).await {
                return Json(ApiResponse::success(global_data));
            }
        }
        Err(e) => {
            tracing::error!("Failed to get by name: {}", e);
        }
    }

    // Return null if nothing found
    Json(ApiResponse::success(Value::Null))
}

// ==================== Health Check ====================

/// GET /health - Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
