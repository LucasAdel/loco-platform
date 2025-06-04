pub mod jobs;
pub mod users;
pub mod health;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

use crate::AppState;

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "loco-platform-backend",
        "timestamp": chrono::Utc::now()
    }))
}

/// Root endpoint with API information
pub async fn root() -> impl IntoResponse {
    Json(json!({
        "name": "Loco Platform API",
        "version": "0.1.0",
        "description": "Professional pharmacy job marketplace API",
        "endpoints": {
            "health": "/health",
            "jobs": "/api/jobs",
            "users": "/api/users"
        }
    }))
}