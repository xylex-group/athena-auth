use axum::{
    extract::State,
    http::StatusCode,
    Json,
    Extension,
};
use serde_json::{json, Value};
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    models::{
        user::{CreateUser, LoginRequest, UserResponse},
        auth::{AuthResponse, Claims},
    },
    utils::{jwt::create_jwt, password::{hash_password, verify_password}},
    AppState,
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<Value>)> {
    // Hash password
    let password_hash = hash_password(&payload.password)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to hash password"})),
            )
        })?;

    // Create user in database
    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let _insert_result = state
        .client
        .insert("users")
        .payload(json!({
            "id": user_id,
            "email": payload.email,
            "password_hash": password_hash,
            "created_at": now,
        }))
        .execute()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to create user: {}", e)})),
            )
        })?;

    let user = UserResponse {
        id: user_id,
        email: payload.email,
        created_at: now,
    };

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    // Fetch user from database
    let result = state
        .client
        .select("users")
        .columns(vec!["id", "email", "password_hash", "created_at"])
        .where_eq("email", payload.email.clone())
        .limit(1)
        .execute()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    // Parse the result
    let rows = &result.rows;

    if rows.is_empty() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ));
    }

    let user = &rows[0];
    let user_id = user.get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Invalid user data"})),
            )
        })?;
    
    let email = user.get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Invalid user data"})),
            )
        })?;
    
    let password_hash = user.get("password_hash")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Invalid user data"})),
            )
        })?;

    // Verify password
    let is_valid = verify_password(&payload.password, password_hash)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Password verification failed"})),
            )
        })?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ));
    }

    // Generate JWT
    let token = create_jwt(user_id, email)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create token"})),
            )
        })?;

    Ok(Json(AuthResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 86400, // 24 hours
    }))
}

pub async fn verify(
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "valid": true,
        "user_id": claims.sub,
        "email": claims.email,
    })))
}

pub async fn me(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponse>, (StatusCode, Json<Value>)> {
    // Fetch user from database
    let result = state
        .client
        .select("users")
        .columns(vec!["id", "email", "created_at"])
        .where_eq("id", claims.sub.clone())
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
            Json(json!({"error": "User not found"})),
        ));
    }

    let user = &rows[0];
    let user_id = user.get("id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Invalid user data"})),
            )
        })?;
    
    let email = user.get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Invalid user data"})),
            )
        })?;
    
    let created_at = user.get("created_at")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Invalid user data"})),
            )
        })?;

    Ok(Json(UserResponse {
        id: user_id,
        email: email.to_string(),
        created_at,
    }))
}
