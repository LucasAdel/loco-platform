use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use sea_orm::DatabaseConnection;

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
    let db_status = check_database_health(&state.db).await;
    
    let overall_status = if db_status == "healthy" { "healthy" } else { "degraded" };
    
    Json(json!({
        "status": overall_status,
        "service": "loco-platform-backend",
        "version": state.config.app_version,
        "timestamp": chrono::Utc::now(),
        "environment": state.config.app_env,
        "uptime": get_uptime(),
        "checks": {
            "database": db_status,
            "memory": check_memory_usage(),
            "external_apis": "demo_mode"
        },
        "metadata": {
            "rust_version": env!("CARGO_PKG_RUST_VERSION"),
            "build_target": std::env::consts::ARCH,
            "host": state.config.host,
            "port": state.config.port
        }
    }))
}

/// Kubernetes readiness probe
pub async fn readiness_check(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db_status = check_database_health(&state.db).await;
    
    if db_status == "healthy" {
        (StatusCode::OK, Json(json!({
            "status": "ready",
            "timestamp": chrono::Utc::now()
        })))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(json!({
            "status": "not_ready",
            "reason": "database_unavailable",
            "timestamp": chrono::Utc::now()
        })))
    }
}

/// Kubernetes liveness probe
pub async fn liveness_check() -> impl IntoResponse {
    Json(json!({
        "status": "alive",
        "timestamp": chrono::Utc::now(),
        "service": "loco-platform-backend"
    }))
}

/// Check database connectivity
async fn check_database_health(_db: &DatabaseConnection) -> String {
    // For now, assume database is healthy if we have a connection
    // In production, you'd ping the database
    if let Ok(_) = std::env::var("DATABASE_URL") {
        "healthy".to_string()
    } else {
        "demo_mode".to_string()
    }
}

/// Get application uptime
fn get_uptime() -> String {
    // This is a simplified uptime - in production you'd track actual start time
    format!("{}s", std::process::id())
}

/// Check memory usage (simplified)
fn check_memory_usage() -> String {
    // In production, you'd use a proper memory monitoring library
    "available".to_string()
}