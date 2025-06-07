pub mod jobs;
pub mod users;
pub mod health;
pub mod auth;
pub mod local_auth;
pub mod websocket;
pub mod search;

use axum::{
    response::{IntoResponse, Json},
};
use serde_json::json;

/// Root endpoint with API information
pub async fn root() -> impl IntoResponse {
    Json(json!({
        "name": "Loco Platform API",
        "version": "0.1.0",
        "description": "Professional pharmacy job marketplace API",
        "documentation": "https://api.locoplatform.com.au/docs",
        "endpoints": {
            "health": {
                "basic": "/health",
                "detailed": "/health/detailed", 
                "ready": "/health/ready",
                "live": "/health/live"
            },
            "api_v1": {
                "jobs": "/api/v1/jobs",
                "users": "/api/v1/users",
                "auth": "/api/v1/auth"
            },
            "legacy": {
                "jobs": "/api/jobs",
                "users": "/api/users"
            }
        },
        "features": [
            "Australian pharmacy jobs",
            "Multi-tenant architecture",
            "Real-time updates",
            "Geospatial search",
            "Professional networking"
        ]
    }))
}

/// Basic health check - uses the one from health.rs
pub use health::health_check;