use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // TODO: Replace with sea_orm::DbErr when database is integrated
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Authorization failed")]
    AuthorizationFailed,
    
    #[error("Resource not found")]
    NotFound,
    
    #[error("Hash operation failed")]
    HashingFailed,
    
    #[error("Token generation failed")]
    TokenGenerationFailed,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
}

// Conversion from shared AppError to backend AppError
impl From<shared::errors::AppError> for AppError {
    fn from(err: shared::errors::AppError) -> Self {
        match err {
            shared::errors::AppError::Database(message) => AppError::Database(message),
            shared::errors::AppError::DatabaseWithContext { message, .. } => AppError::Database(message),
            shared::errors::AppError::Validation { field, message, .. } => {
                AppError::Validation(format!("{}: {}", field, message))
            },
            shared::errors::AppError::Authentication { reason, .. } => {
                tracing::warn!("Authentication error: {}", reason);
                AppError::AuthenticationFailed
            },
            shared::errors::AppError::Authorisation { .. } => AppError::AuthorizationFailed,
            shared::errors::AppError::NotFound => AppError::NotFound,
            shared::errors::AppError::NotFoundDetailed { resource_type, id, .. } => {
                tracing::info!("Resource not found: {} with ID {}", resource_type, id);
                AppError::NotFound
            },
            shared::errors::AppError::InvalidInput { message, .. } => AppError::Validation(message),
            shared::errors::AppError::Internal { message, error_id, .. } => {
                tracing::error!("Internal error ({}): {}", error_id, message);
                AppError::Internal(message)
            },
            _ => AppError::Internal(err.to_string()),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            AppError::Database(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred".to_string())
            }
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::AuthenticationFailed => (StatusCode::UNAUTHORIZED, "Authentication failed".to_string()),
            AppError::AuthorizationFailed => (StatusCode::FORBIDDEN, "Authorization failed".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::HashingFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing failed".to_string()),
            AppError::TokenGenerationFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Token generation failed".to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token".to_string()),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::Configuration(msg) => {
                tracing::error!("Configuration error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Service configuration error".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
            "timestamp": chrono::Utc::now()
        }));

        (status, body).into_response()
    }
}