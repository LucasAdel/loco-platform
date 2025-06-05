use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    services::AuthService,
    AppState, AppError,
};

/// JWT Authentication middleware
pub async fn jwt_auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Extract token from header
    let token = AuthService::extract_token_from_header(auth_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Validate token
    let claims = AuthService::validate_token(token, &state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Add user info to request extensions for use in handlers
    request.extensions_mut().insert(CurrentUser {
        id: claims.sub,
        email: claims.email,
        user_type: claims.user_type,
    });
    
    Ok(next.run(request).await)
}

/// Current authenticated user information
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub user_type: String,
}

impl CurrentUser {
    /// Check if user is an admin
    pub fn is_admin(&self) -> bool {
        self.user_type == "SuperAdmin"
    }
    
    /// Check if user is an employer
    pub fn is_employer(&self) -> bool {
        self.user_type == "Employer"
    }
    
    /// Check if user is a professional
    pub fn is_professional(&self) -> bool {
        self.user_type == "Professional"
    }
}

/// Optional JWT authentication middleware (doesn't reject on missing token)
pub async fn optional_jwt_auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    // Try to get Authorization header
    if let Some(auth_header) = headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        // Try to extract and validate token
        if let Ok(token) = AuthService::extract_token_from_header(auth_header) {
            if let Ok(claims) = AuthService::validate_token(token, &state.config.jwt_secret) {
                // Add user info to request extensions
                request.extensions_mut().insert(CurrentUser {
                    id: claims.sub,
                    email: claims.email,
                    user_type: claims.user_type,
                });
            }
        }
    }
    
    next.run(request).await
}