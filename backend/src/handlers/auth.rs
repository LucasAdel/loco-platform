use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    services::AuthService,
    entities::{user, tenant_users},
    AppState, AppError,
};
use shared::types::{UserType, UserId};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub user_type: UserType,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_type: String,
    pub is_active: bool,
}

/// User login endpoint
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Login attempt for email: {}", request.email);
    
    // For demo mode, validate against demo credentials
    if request.email == "admin@loco-platform.com" && request.password == "demo-password" {
        let user_id = Uuid::new_v4();
        
        // Generate JWT token
        let token = AuthService::generate_token(
            user_id,
            request.email.clone(),
            "Employer".to_string(),
            &state.config.jwt_secret,
        )?;
        
        let response = AuthResponse {
            token,
            user: UserInfo {
                id: user_id,
                email: request.email,
                first_name: "Demo".to_string(),
                last_name: "Admin".to_string(),
                user_type: "Employer".to_string(),
                is_active: true,
            },
        };
        
        tracing::info!("Successful login for demo admin");
        return Ok((StatusCode::OK, Json(response)));
    }
    
    if request.email == "pharmacist@example.com.au" && request.password == "demo-password" {
        let user_id = Uuid::new_v4();
        
        // Generate JWT token
        let token = AuthService::generate_token(
            user_id,
            request.email.clone(),
            "Professional".to_string(),
            &state.config.jwt_secret,
        )?;
        
        let response = AuthResponse {
            token,
            user: UserInfo {
                id: user_id,
                email: request.email,
                first_name: "Michael".to_string(),
                last_name: "Chen".to_string(),
                user_type: "Professional".to_string(),
                is_active: true,
            },
        };
        
        tracing::info!("Successful login for demo professional");
        return Ok((StatusCode::OK, Json(response)));
    }
    
    // Invalid credentials
    tracing::warn!("Failed login attempt for email: {}", request.email);
    Err(AppError::AuthenticationFailed)
}

/// User registration endpoint
pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Registration attempt for email: {}", request.email);
    
    // Validate input
    if request.email.is_empty() || request.password.len() < 8 {
        return Err(AppError::Validation(
            "Email is required and password must be at least 8 characters".to_string(),
        ));
    }
    
    // Hash password
    let password_hash = AuthService::hash_password(&request.password)?;
    
    // For demo mode, create a demo user
    let user_id = Uuid::new_v4();
    
    // Generate JWT token
    let token = AuthService::generate_token(
        user_id,
        request.email.clone(),
        format!("{:?}", request.user_type),
        &state.config.jwt_secret,
    )?;
    
    let response = AuthResponse {
        token,
        user: UserInfo {
            id: user_id,
            email: request.email.clone(),
            first_name: request.first_name.clone(),
            last_name: request.last_name.clone(),
            user_type: format!("{:?}", request.user_type),
            is_active: true,
        },
    };
    
    tracing::info!("Successful registration for email: {}", request.email);
    Ok((StatusCode::CREATED, Json(response)))
}

/// Token refresh endpoint
pub async fn refresh_token(
    State(state): State<AppState>,
    // TODO: Extract current user from JWT middleware
) -> Result<impl IntoResponse, AppError> {
    // For now, return a demo response
    Ok(Json(json!({
        "message": "Token refresh endpoint - not yet implemented",
        "status": "coming_soon"
    })))
}

/// User logout endpoint
pub async fn logout() -> Result<impl IntoResponse, AppError> {
    // For JWT-based auth, logout is typically handled client-side
    // by removing the token. Server-side blacklisting can be added later.
    
    Ok(Json(json!({
        "message": "Logged out successfully",
        "status": "success"
    })))
}

/// Get current user profile
pub async fn get_profile(
    State(state): State<AppState>,
    // TODO: Extract current user from JWT middleware
) -> Result<impl IntoResponse, AppError> {
    // For now, return a demo profile
    Ok(Json(json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "email": "demo@loco-platform.com",
        "first_name": "Demo",
        "last_name": "User",
        "user_type": "Professional",
        "is_active": true,
        "created_at": chrono::Utc::now(),
        "profile": {
            "bio": "Experienced pharmacist in Adelaide",
            "skills": ["Dispensing", "Customer Service", "Clinical Pharmacy"],
            "experience_years": 5
        }
    })))
}

/// Update user profile
pub async fn update_profile(
    State(state): State<AppState>,
    Json(profile_data): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Profile update request received");
    
    // For demo mode, just return success
    Ok(Json(json!({
        "message": "Profile updated successfully",
        "status": "success",
        "updated_fields": profile_data
    })))
}