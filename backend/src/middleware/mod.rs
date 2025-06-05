pub mod auth;
pub mod cors;
pub mod logging;
pub mod supabase_auth;
pub mod jwt_auth;
pub mod validation;

// Re-export middleware
pub use auth::auth_middleware;
pub use cors::cors_layer;
pub use logging::request_tracing;
pub use jwt_auth::{jwt_auth_middleware, optional_jwt_auth_middleware, CurrentUser};
pub use validation::{ValidatedJson, validation_middleware, rate_limit_middleware};