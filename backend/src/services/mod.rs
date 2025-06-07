pub mod job_service;
pub mod user_service;
pub mod auth_service;
pub mod location_service;
pub mod supabase_auth;
pub mod search_service;

// Re-export services
pub use job_service::JobService;
pub use user_service::UserService;
pub use auth_service::AuthService;
pub use location_service::LocationService;
pub use supabase_auth::SupabaseAuthService;
pub use search_service::SearchService;