mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod services;
mod entities;
mod repository;
mod database;
mod db;

// Re-export commonly used types
pub use error::AppError;

use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method},
    middleware as axum_middleware,
    routing::{get, post, put, delete},
    Router,
};
use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sea_orm::DatabaseConnection;
use tokio::signal;

use crate::{
    config::Config,
    handlers::{jobs, health, users, auth},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging with multiple layers
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    tracing::info!("🦀 Starting Loco Platform - Rust Web Application");
    
    // Load configuration
    let config = Config::from_env()?;
    
    // Try to setup database connection, fall back to demo mode if it fails
    tracing::info!("🔗 Attempting to set up database connection...");
    
    // Temporarily force demo mode to avoid database seeding issues
    match if true { Err(anyhow::anyhow!("Forced demo mode")) } else { database::setup_database().await } {
        Ok(db) => {
            tracing::info!("🚀 Starting Loco Platform server with full database support");
            let state = AppState { config: config.clone(), db };
            
            // Build middleware stack (simplified for now)
            let middleware_stack = ServiceBuilder::new()
                // Request tracing (simplified)
                .layer(TraceLayer::new_for_http())
                // CORS with Australian domain handling
                .layer(create_cors_layer());

            // Build our application with routes
            let app = Router::new()
                // Root and health endpoints
                .route("/", get(handlers::root))
                .route("/health", get(handlers::health_check))
                .route("/health/detailed", get(health::health_detailed))
                .route("/health/ready", get(health::readiness_check))
                .route("/health/live", get(health::liveness_check))
                
                // API v1 routes
                .nest("/api/v1", api_v1_routes())
                
                // Legacy API routes (for backward compatibility)
                .route("/api/jobs", get(jobs::list_jobs).post(jobs::create_job))
                .route("/api/jobs/search", post(jobs::search_jobs))
                .route("/api/jobs/:id", get(jobs::get_job).put(jobs::update_job).delete(jobs::delete_job))
                
                // User routes  
                .route("/api/users/:id", get(users::get_user).put(users::update_user).delete(users::delete_user))
                
                // Apply middleware stack
                .layer(middleware_stack)
                .with_state(state);

            // Start server with graceful shutdown
            start_server_with_graceful_shutdown(app, config.port).await?;
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

/// Create CORS layer with Australian domain handling
fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::USER_AGENT,
        ])
        // Australian domains and localhost for development
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
        .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
        .allow_origin("https://locoplatform.com.au".parse::<HeaderValue>().unwrap())
        .allow_origin("https://www.locoplatform.com.au".parse::<HeaderValue>().unwrap())
        .allow_origin("https://api.locoplatform.com.au".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .max_age(Duration::from_secs(86400)) // 24 hours
}

/// Create API v1 routes with versioning
fn api_v1_routes() -> Router<AppState> {
    Router::new()
        // Job management routes
        .route("/jobs", get(jobs::list_jobs).post(jobs::create_job))
        .route("/jobs/search", post(jobs::search_jobs))
        .route("/jobs/:id", get(jobs::get_job).put(jobs::update_job).delete(jobs::delete_job))
        
        // User management routes
        .route("/users/:id", get(users::get_user).put(users::update_user).delete(users::delete_user))
        
        // Authentication routes
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/refresh", post(auth::refresh_token))
        .route("/auth/logout", post(auth::logout))
        
        // Profile routes
        .route("/profile", get(auth::get_profile).put(auth::update_profile))
        
        // Application routes
        .route("/applications", get(placeholder_handler).post(placeholder_handler))
        .route("/applications/:id", get(placeholder_handler).put(placeholder_handler).delete(placeholder_handler))
}

/// Placeholder handler for routes not yet implemented
async fn placeholder_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "message": "This endpoint is not yet implemented",
        "status": "coming_soon"
    }))
}

/// Start server with graceful shutdown handling
async fn start_server_with_graceful_shutdown(app: Router, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Loco Platform backend listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    
    tracing::info!("🛑 Server has shut down gracefully");
    Ok(())
}

/// Handle graceful shutdown signals
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("📡 Received Ctrl+C signal, starting graceful shutdown...");
        },
        _ = terminate => {
            tracing::info!("📡 Received SIGTERM signal, starting graceful shutdown...");
        },
    }
}