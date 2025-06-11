pub mod jobs;
pub mod enhanced_jobs;
pub mod users;
pub mod health;
pub mod auth;
pub mod local_auth;
pub mod websocket;
pub mod search;
pub mod applications;

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
            "auth": {
                "login": "/api/v1/auth/login",
                "register": "/api/v1/auth/register",
                "logout": "/api/v1/auth/logout",
                "refresh": "/api/v1/auth/refresh",
                "profile": "/api/v1/profile",
                "local": {
                    "login": "/api/v1/auth/local/login",
                    "register": "/api/v1/auth/local/register",
                    "verify": "/api/v1/auth/local/verify"
                }
            },
            "jobs": {
                "list": "/api/v1/jobs",
                "create": "/api/v1/jobs",
                "get": "/api/v1/jobs/:id",
                "update": "/api/v1/jobs/:id",
                "delete": "/api/v1/jobs/:id",
                "search": "/api/v1/jobs/search",
                "enhanced": {
                    "list": "/api/v1/jobs/enhanced",
                    "search": "/api/v1/jobs/enhanced/search",
                    "nearby": "/api/v1/jobs/enhanced/nearby",
                    "stats": "/api/v1/jobs/enhanced/stats",
                    "my_jobs": "/api/v1/jobs/enhanced/my",
                    "my_stats": "/api/v1/jobs/enhanced/my/stats"
                }
            },
            "users": {
                "get": "/api/v1/users/:id",
                "update": "/api/v1/users/:id",
                "delete": "/api/v1/users/:id"
            },
            "applications": {
                "list": "/api/v1/applications",
                "create": "/api/v1/applications",
                "get": "/api/v1/applications/:id",
                "update": "/api/v1/applications/:id"
            },
            "search": {
                "advanced": "/api/v1/search/advanced",
                "quick": "/api/v1/search/quick",
                "suggestions": "/api/v1/search/suggestions",
                "trending": "/api/v1/search/trending",
                "recommendations": "/api/v1/search/recommendations"
            },
            "websocket": "/ws",
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
            "Professional networking",
            "JWT authentication",
            "Enhanced job search",
            "Application tracking"
        ]
    }))
}

/// Basic health check - uses the one from health.rs
pub use health::health_check;