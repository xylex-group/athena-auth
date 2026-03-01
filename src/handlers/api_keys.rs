use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
    Extension,
};
use serde_json::{json, Value};
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    models::{
        api_key::{CreateApiKey, ApiKeyResponse, ApiKeyInfo},
        auth::Claims,
    },
    utils::{password::hash_password, api_key_gen::generate_api_key},
    AppState,
};

pub async fn create_api_key(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateApiKey>,
) -> Result<(StatusCode, Json<ApiKeyResponse>), (StatusCode, Json<Value>)> {
    // Generate API key
    let api_key = generate_api_key();
    let key_hash = hash_password(&api_key)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to hash API key"})),
            )
        })?;

    let key_id = Uuid::new_v4();
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid user ID"})),
            )
        })?;
    let now = chrono::Utc::now();

    // Store in api_keys table (or public table as mentioned in requirements)
    let _insert_result = state
        .client
        .insert("api_keys")
        .payload(json!({
            "id": key_id,
            "user_id": user_id,
            "key_hash": key_hash,
            "name": payload.name,
            "created_at": now,
            "last_used_at": null,
            "expires_at": payload.expires_at,
            "is_active": true,
        }))
        .execute()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to create API key: {}", e)})),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(ApiKeyResponse {
            id: key_id,
            name: payload.name,
            key: api_key,
            created_at: now,
            expires_at: payload.expires_at,
        }),
    ))
}

pub async fn list_api_keys(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ApiKeyInfo>>, (StatusCode, Json<Value>)> {
    let result = state
        .client
        .select("api_keys")
        .columns(vec!["id", "name", "created_at", "last_used_at", "expires_at", "is_active"])
        .where_eq("user_id", claims.sub.clone())
        .execute()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    let rows = &result.rows;
    
    let api_keys: Vec<ApiKeyInfo> = rows
        .iter()
        .filter_map(|row| {
            let id = row.get("id")
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())?;
            
            let name = row.get("name")
                .and_then(|v| v.as_str())?
                .to_string();
            
            let created_at = row.get("created_at")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc))?;
            
            let last_used_at = row.get("last_used_at")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc));
            
            let expires_at = row.get("expires_at")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc));
            
            let is_active = row.get("is_active")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            Some(ApiKeyInfo {
                id,
                name,
                created_at,
                last_used_at,
                expires_at,
                is_active,
            })
        })
        .collect();

    Ok(Json(api_keys))
}

pub async fn revoke_api_key(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Path(key_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    // Verify the API key belongs to the user
    let result = state
        .client
        .select("api_keys")
        .columns(vec!["user_id"])
        .where_eq("id", key_id.to_string())
        .limit(1)
        .execute()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    let rows = &result.rows;

    if rows.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "API key not found"})),
        ));
    }

    let owner_id = rows[0].get("user_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Invalid API key data"})),
            )
        })?;

    if owner_id != claims.sub {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "You don't have permission to revoke this API key"})),
        ));
    }

    // Update the API key to mark it as inactive
    state
        .client
        .update("api_keys", Some(key_id.to_string()))
        .payload(json!({
            "is_active": false,
        }))
        .execute()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to revoke API key: {}", e)})),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
