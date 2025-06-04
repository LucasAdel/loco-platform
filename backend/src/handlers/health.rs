use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use serde_json::json;

use crate::AppState;

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "loco-platform-backend",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now(),
        "environment": "development"
    }))
}

/// Detailed health check with dependencies
pub async fn health_detailed(
    State(state): State<AppState>,
) -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "loco-platform-backend",
        "version": state.config.app_version,
        "timestamp": chrono::Utc::now(),
        "environment": state.config.app_env,
        "checks": {
            "database": "demo_mode",
            "redis": "not_configured",
            "external_apis": "demo_mode"
        },
        "demo_mode": true
    }))
}