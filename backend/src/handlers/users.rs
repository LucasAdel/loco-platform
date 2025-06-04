use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use uuid::Uuid;

use crate::{AppState, AppError};

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