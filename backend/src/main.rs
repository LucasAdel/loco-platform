mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod services;
mod entities;
mod repository;
mod database;

// Re-export commonly used types
pub use error::AppError;

use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method},
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber;
use sea_orm::DatabaseConnection;

use crate::{
    config::Config,
    handlers::{jobs, health, users},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Load configuration
    let config = Config::from_env()?;
    
    // Try to setup database connection, fall back to demo mode if it fails
    tracing::info!("🔗 Attempting to set up database connection...");
    
    match database::setup_database().await {
        Ok(db) => {
            tracing::info!("🚀 Starting Loco Platform server with full database support");
            let state = AppState { config, db };
            
            // Build our application with routes
            let app = Router::new()
                // Root and health endpoints
                .route("/", get(handlers::root))
                .route("/health", get(handlers::health_check))
                .route("/health/detailed", get(health::health_detailed))
                
                // Job routes
                .route("/api/jobs", get(jobs::list_jobs).post(jobs::create_job))
                .route("/api/jobs/search", post(jobs::search_jobs))
                .route("/api/jobs/:id", get(jobs::get_job).put(jobs::update_job).delete(jobs::delete_job))
                
                // User routes  
                .route("/api/users/:id", get(users::get_user).put(users::update_user).delete(users::delete_user))
                
                // Simple CORS layer for demo
                .layer(CorsLayer::permissive())
                .with_state(state);

            // Run the server
            let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
            tracing::info!("🚀 Loco Platform backend listening on {}", addr);
            
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, app).await?;
        },
        Err(e) => {
            tracing::warn!("⚠️ Database connection failed: {}", e);
            tracing::info!("🚀 Starting Loco Platform server in DEMO MODE (no database)");
            
            // Demo mode - create a simple state without database
            #[derive(Clone)]
            struct DemoState {
                config: Config,
            }
            
            let demo_state = DemoState { config };
            
            // Build demo application with limited routes
            let app = Router::new()
                .route("/", get(demo_root))
                .route("/health", get(demo_health))
                .route("/api/jobs", get(demo_jobs))
                .layer(CorsLayer::permissive())
                .with_state(demo_state);

            let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
            tracing::info!("🚀 Loco Platform backend (DEMO MODE) listening on {}", addr);
            
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}

// Demo mode handlers
async fn demo_root() -> &'static str {
    "Loco Platform Demo - Backend Running (No Database)"
}

async fn demo_health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "mode": "demo",
        "message": "Server running without database"
    }))
}

async fn demo_jobs() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "jobs": [
            {
                "id": "demo-1",
                "title": "Demo Pharmacist Position",
                "description": "This is a demo job listing",
                "location": "Sydney, NSW",
                "hourly_rate": 45.00,
                "job_type": "Pharmacist",
                "is_urgent": false
            }
        ],
        "total": 1,
        "message": "Demo data - database not connected"
    }))
}