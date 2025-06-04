use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{services::AuthService, AppState, AppError};

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub email: String,
    pub user_type: String,
}

/// Authentication middleware
pub async fn auth_middleware(
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
    
    // Create authenticated user and add to request extensions
    let auth_user = AuthenticatedUser {
        user_id: claims.sub,
        email: claims.email,
        user_type: claims.user_type,
    };
    
    request.extensions_mut().insert(auth_user);
    
    Ok(next.run(request).await)
}

/// Optional authentication middleware (doesn't fail if no token)
pub async fn optional_auth_middleware(
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
                let auth_user = AuthenticatedUser {
                    user_id: claims.sub,
                    email: claims.email,
                    user_type: claims.user_type,
                };
                
                request.extensions_mut().insert(auth_user);
            }
        }
    }
    
    next.run(request).await
}