use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    services::{AuthService, UserService},
    AppState, AppError,
};
use shared::types::{User, UserType};

#[derive(Debug, Deserialize)]
pub struct LocalLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LocalRegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub user_type: UserType,
}

#[derive(Debug, Serialize)]
pub struct LocalAuthResponse {
    pub access_token: String,
    pub user: User,
}

/// Local login endpoint (without Supabase)
pub async fn local_login(
    State(state): State<AppState>,
    Json(request): Json<LocalLoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Local login attempt for email: {}", request.email);
    
    // In demo mode, create a mock user
    let user = if state.demo_mode {
        // Demo mode: accept any email/password combination with demo credentials
        if request.password.len() < 4 {
            return Err(AppError::Validation("Password too short for demo".to_string()));
        }
        
        User {
            id: Uuid::new_v4(),
            email: request.email.clone(),
            first_name: "Demo".to_string(),
            last_name: "User".to_string(),
            phone: None,
            user_type: UserType::Professional,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    } else {
        // Production mode would verify credentials from database
        // For now, return error as database is not connected
        return Err(AppError::Configuration("Database authentication not available".to_string()));
    };
    
    // Generate JWT token
    let jwt_secret = &state.config.jwt_secret;
        
    let token = AuthService::generate_token(
        user.id,
        user.email.clone(),
        format!("{:?}", user.user_type),
        jwt_secret,
    )?;
    
    // Create auth cookie
    let cookie = format!(
        "auth-token={}; HttpOnly; Secure; SameSite=Lax; Max-Age=86400; Path=/",
        token
    );
    
    let response = LocalAuthResponse {
        access_token: token,
        user,
    };
    
    tracing::info!("Successful local login for user: {}", response.user.id);
    
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(response),
    ))
}

/// Local registration endpoint (without Supabase)
pub async fn local_register(
    State(state): State<AppState>,
    Json(request): Json<LocalRegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Local registration attempt for email: {}", request.email);
    
    // Validate input
    if request.email.is_empty() || request.password.len() < 8 {
        return Err(AppError::Validation(
            "Email is required and password must be at least 8 characters".to_string(),
        ));
    }
    
    // Hash password
    let password_hash = AuthService::hash_password(&request.password)?;
    
    // Create user
    let user = if state.demo_mode {
        // Demo mode: create mock user
        User {
            id: Uuid::new_v4(),
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            phone: request.phone,
            user_type: request.user_type,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    } else {
        // Production mode would save to database
        // For now, return error as database is not connected
        return Err(AppError::Configuration("Database registration not available".to_string()));
    };
    
    // Generate JWT token
    let jwt_secret = &state.config.jwt_secret;
        
    let token = AuthService::generate_token(
        user.id,
        user.email.clone(),
        format!("{:?}", user.user_type),
        jwt_secret,
    )?;
    
    // Create auth cookie
    let cookie = format!(
        "auth-token={}; HttpOnly; Secure; SameSite=Lax; Max-Age=86400; Path=/",
        token
    );
    
    let response = LocalAuthResponse {
        access_token: token,
        user,
    };
    
    tracing::info!("Successful local registration for user: {}", response.user.id);
    
    Ok((
        StatusCode::CREATED,
        [(header::SET_COOKIE, cookie)],
        Json(response),
    ))
}

/// Local logout endpoint
pub async fn local_logout() -> Result<impl IntoResponse, AppError> {
    // Clear auth cookie
    let clear_cookie = "auth-token=; HttpOnly; Secure; SameSite=Lax; Max-Age=0; Path=/";
    
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, clear_cookie)],
        Json(json!({
            "message": "Logged out successfully",
            "status": "success"
        })),
    ))
}

/// Verify token endpoint
pub async fn verify_token(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // Extract token from Authorization header
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::AuthenticationFailed)?;
    
    let token = AuthService::extract_token_from_header(auth_header)?;
    
    // Validate token
    let jwt_secret = &state.config.jwt_secret;
        
    let claims = AuthService::validate_token(token, jwt_secret)?;
    
    Ok(Json(json!({
        "valid": true,
        "user_id": claims.sub,
        "email": claims.email,
        "user_type": claims.user_type,
        "expires_at": claims.exp,
    })))
}

/// Refresh token endpoint
pub async fn refresh_token(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // Extract current token
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::AuthenticationFailed)?;
    
    let token = AuthService::extract_token_from_header(auth_header)?;
    
    // Validate current token
    let jwt_secret = &state.config.jwt_secret;
        
    let claims = AuthService::validate_token(token, jwt_secret)?;
    
    // Generate new token with extended expiration
    let new_token = AuthService::generate_token(
        claims.sub,
        claims.email,
        claims.user_type,
        jwt_secret,
    )?;
    
    // Create new auth cookie
    let cookie = format!(
        "auth-token={}; HttpOnly; Secure; SameSite=Lax; Max-Age=86400; Path=/",
        new_token
    );
    
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(json!({
            "access_token": new_token,
            "expires_in": 86400, // 24 hours
        })),
    ))
}