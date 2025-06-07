use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{AppState, AppError};

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}

/// List all users (admin only)
pub async fn list_users(
    State(_state): State<AppState>,
    Query(params): Query<ListUsersQuery>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    
    // Demo mode - return mock user list
    Ok(Json(json!({
        "users": [
            {
                "id": Uuid::new_v4(),
                "email": "admin@loco-platform.com",
                "first_name": "Admin",
                "last_name": "User",
                "user_type": "SuperAdmin",
                "is_active": true,
                "created_at": "2025-01-01T00:00:00Z"
            },
            {
                "id": Uuid::new_v4(),
                "email": "employer@pharmacy.com",
                "first_name": "John",
                "last_name": "Smith",
                "user_type": "Employer",
                "is_active": true,
                "created_at": "2025-01-02T00:00:00Z"
            },
            {
                "id": Uuid::new_v4(),
                "email": "pharmacist@example.com",
                "first_name": "Jane",
                "last_name": "Doe",
                "user_type": "Professional",
                "is_active": true,
                "created_at": "2025-01-03T00:00:00Z"
            }
        ],
        "total": 3,
        "page": page,
        "limit": limit,
        "has_more": false
    })))
}

/// Get user profile
pub async fn get_user(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Demo mode: Use service
    match crate::services::UserService::get_user_by_id(user_id).await? {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::NotFound),
    }
}

/// Update user profile
pub async fn update_user(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UserUpdateRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Demo mode: Use service
    match crate::services::UserService::update_user(
        user_id,
        request.first_name,
        request.last_name,
        request.phone,
    ).await? {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::NotFound),
    }
}

/// Delete user account
pub async fn delete_user(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Demo mode: Use service (deactivation)
    if crate::services::UserService::deactivate_user(user_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

#[derive(serde::Deserialize)]
pub struct UserUpdateRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}