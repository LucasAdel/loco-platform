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
    handlers::{jobs, enhanced_jobs, health, users, auth, local_auth, websocket, search, applications, root as handlers},
    services::ApplicationService,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: DatabaseConnection,
    pub application_service: ApplicationService,
    pub demo_mode: bool,
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
    
    // Setup database connection - no fallbacks
    tracing::info!("🔗 Setting up database connection...");
    
    let db = database::setup_database().await
        .map_err(|e| anyhow::anyhow!("Database connection failed: {}. Please ensure PostgreSQL is running and database is configured.", e))?;
        
    tracing::info!("✅ Database connection established successfully");
    
    let application_service = ApplicationService::new(db.clone());
    let state = AppState { 
        config: config.clone(), 
        db, 
        application_service,
        demo_mode: true, // Enable demo mode for development
    };
    
    // Build middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(create_cors_layer());

    // Build application with routes
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
        
        // WebSocket routes
        .route("/ws", get(websocket::websocket_handler))
        
        // Apply middleware stack
        .layer(middleware_stack)
        .with_state(state);

    tracing::info!("🚀 Starting Loco Platform server with full database support");
    
    // Start server with graceful shutdown
    start_server_with_graceful_shutdown(app, config.port).await?;

    Ok(())
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
        .allow_origin("http://localhost:3070".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:3080".parse::<HeaderValue>().unwrap())
        .allow_origin("http://127.0.0.1:3070".parse::<HeaderValue>().unwrap())
        .allow_origin("http://127.0.0.1:3080".parse::<HeaderValue>().unwrap())
        .allow_origin("https://locoplatform.com.au".parse::<HeaderValue>().unwrap())
        .allow_origin("https://www.locoplatform.com.au".parse::<HeaderValue>().unwrap())
        .allow_origin("https://api.locoplatform.com.au".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .max_age(Duration::from_secs(86400)) // 24 hours
}

/// Create API v1 routes with versioning
fn api_v1_routes() -> Router<AppState> {
    Router::new()
        // Public routes (no authentication required)
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/forgot-password", post(auth::forgot_password))
        .route("/auth/verify-otp", post(auth::verify_otp))
        .route("/auth/oauth/:provider", get(auth::oauth_signin))
        
        // Local auth routes (JWT-based, no Supabase)
        .route("/auth/local/login", post(local_auth::local_login))
        .route("/auth/local/register", post(local_auth::local_register))
        .route("/auth/local/logout", post(local_auth::local_logout))
        .route("/auth/local/verify", get(local_auth::verify_token))
        .route("/auth/local/refresh", post(local_auth::refresh_token))
        .route("/jobs", get(jobs::list_jobs)) // Public job listing
        .route("/jobs/search", post(jobs::search_jobs)) // Public job search
        .route("/jobs/:id", get(jobs::get_job)) // Public job details
        
        // Enhanced job routes
        .route("/jobs/enhanced", get(enhanced_jobs::list_jobs)) // Enhanced job listing with better filtering
        .route("/jobs/enhanced/search", get(enhanced_jobs::search_jobs)) // Enhanced text search
        .route("/jobs/enhanced/nearby", get(enhanced_jobs::find_jobs_nearby)) // Location-based search
        .route("/jobs/enhanced/stats", get(enhanced_jobs::get_global_job_statistics)) // Global statistics
        
        // Advanced search routes
        .route("/search/advanced", post(search::advanced_search))
        .route("/search/quick", get(search::quick_search))
        .route("/search/suggestions", get(search::search_suggestions))
        .route("/search/trending", get(search::trending_searches))
        .route("/search/recommendations", get(search::job_recommendations))
        
        // Protected routes (require authentication)
        .route("/jobs", post(jobs::create_job))
        .route("/jobs/:id", put(jobs::update_job).delete(jobs::delete_job))
        
        // Enhanced job routes (protected)
        .route("/jobs/enhanced", post(enhanced_jobs::create_job)) // Create job with enhanced features
        .route("/jobs/enhanced/:id", get(enhanced_jobs::get_job).delete(enhanced_jobs::delete_job)) // Enhanced job operations
        .route("/jobs/enhanced/:id/status", put(enhanced_jobs::update_job_status)) // Update job status
        .route("/jobs/enhanced/my", get(enhanced_jobs::get_my_jobs)) // Get user's jobs
        .route("/jobs/enhanced/my/stats", get(enhanced_jobs::get_job_statistics)) // User's job statistics
        
        // User management routes (protected)
        .route("/users/:id", get(users::get_user).put(users::update_user).delete(users::delete_user))
        
        // Authentication routes
        .route("/auth/refresh", post(auth::refresh_token))
        
        // Profile routes (protected)
        .route("/profile", get(auth::get_profile).put(auth::update_profile))
        
        // Application routes (protected)
        .route("/applications", get(applications::list_applications).post(applications::create_application))
        .route("/applications/:id", get(applications::get_application).put(applications::update_application).delete(applications::delete_application))
        .route("/applications/:id/status", put(applications::update_application_status))
        .route("/applications/:id/withdraw", put(applications::withdraw_application))
        .route("/applications/stats", get(applications::get_application_stats))
        .route("/jobs/:id/applications", get(applications::get_job_applications))
        .route("/users/:id/applications", get(applications::get_user_applications))
        
        // Protected search routes
        .route("/search/saved", get(search::get_saved_searches))
        .route("/search/save", post(search::save_search))
}

/// Legacy API routes (backward compatibility)
fn legacy_routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", get(jobs::list_jobs))
        .route("/jobs/:id", get(jobs::get_job))
        .route("/users/:id", get(users::get_user))
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