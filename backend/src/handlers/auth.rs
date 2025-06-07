use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    services::supabase_auth::{SupabaseAuthService, SignUpInput, SignInInput, UpdateUserInput},
    AppState, AppError,
};
use shared::types::UserType;
use shared::supabase::SupabaseConfig;

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
    pub tenant_slug: Option<String>, // For multi-tenant support
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub user_type: Option<String>,
    pub tenant_id: Option<Uuid>,
}

/// User login endpoint with Supabase
pub async fn login(
    State(_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Login attempt for email: {}", request.email);
    
    // Initialize Supabase client
    let supabase_config = SupabaseConfig::from_env()
        .map_err(|e| AppError::Configuration(e))?;
    let auth_service = SupabaseAuthService::new(supabase_config);
    
    // Sign in with Supabase
    let sign_in_input = SignInInput {
        email: request.email.clone(),
        password: request.password,
    };
    
    let auth_response = auth_service
        .sign_in(sign_in_input)
        .await
        .map_err(|e| {
            tracing::error!("Supabase sign in failed: {}", e);
            AppError::AuthenticationFailed
        })?;
    
    // Extract user metadata
    let user_metadata = &auth_response.user.user_metadata;
    let app_metadata = &auth_response.user.app_metadata;
    
    // Create auth cookie first (before moving values)
    let cookie = SupabaseAuthService::create_auth_cookie(&auth_response);
    
    let response = AuthResponse {
        access_token: auth_response.access_token,
        refresh_token: auth_response.refresh_token,
        expires_in: auth_response.expires_in,
        user: UserInfo {
            id: auth_response.user.id,
            email: auth_response.user.email,
            first_name: user_metadata.first_name.clone(),
            last_name: user_metadata.last_name.clone(),
            user_type: Some(app_metadata.role.clone()),
            tenant_id: Some(app_metadata.tenant_id),
        },
    };
    
    tracing::info!("Successful login for user: {}", response.user.id);
    
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(response),
    ))
}

/// User registration endpoint with Supabase
pub async fn register(
    State(_state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Registration attempt for email: {}", request.email);
    
    // Validate input
    if request.email.is_empty() || request.password.len() < 8 {
        return Err(AppError::Validation(
            "Email is required and password must be at least 8 characters".to_string(),
        ));
    }
    
    // Initialize Supabase client
    let supabase_config = SupabaseConfig::from_env()
        .map_err(|e| AppError::Configuration(e))?;
    let auth_service = SupabaseAuthService::new(supabase_config);
    
    // Prepare user metadata
    let user_metadata = json!({
        "first_name": request.first_name,
        "last_name": request.last_name,
        "phone": request.phone,
    });
    
    // For multi-tenant support, we'll set app metadata after user creation
    // This typically requires admin privileges
    
    // Sign up with Supabase
    let sign_up_input = SignUpInput {
        email: request.email.clone(),
        password: request.password,
        data: Some(user_metadata),
    };
    
    let auth_response = auth_service
        .sign_up(sign_up_input)
        .await
        .map_err(|e| {
            tracing::error!("Supabase sign up failed: {}", e);
            AppError::Validation(format!("Registration failed: {}", e))
        })?;
    
    // Create auth cookie first (before moving values)
    let cookie = SupabaseAuthService::create_auth_cookie(&auth_response);
    
    let response = AuthResponse {
        access_token: auth_response.access_token,
        refresh_token: auth_response.refresh_token,
        expires_in: auth_response.expires_in,
        user: UserInfo {
            id: auth_response.user.id,
            email: auth_response.user.email,
            first_name: Some(request.first_name),
            last_name: Some(request.last_name),
            user_type: Some(format!("{:?}", request.user_type)),
            tenant_id: None, // Will be set later
        },
    };
    
    tracing::info!("Successful registration for user: {}", response.user.id);
    
    Ok((
        StatusCode::CREATED,
        [(header::SET_COOKIE, cookie)],
        Json(response),
    ))
}

/// Token refresh endpoint
pub async fn refresh_token(
    State(_state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // Extract refresh token from request
    let refresh_token = headers
        .get("X-Refresh-Token")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::AuthenticationFailed)?;
    
    // Initialize Supabase client
    let supabase_config = SupabaseConfig::from_env()
        .map_err(|e| AppError::Configuration(e))?;
    let auth_service = SupabaseAuthService::new(supabase_config);
    
    // Refresh the token
    let auth_response = auth_service
        .refresh_token(refresh_token)
        .await
        .map_err(|e| {
            tracing::error!("Token refresh failed: {}", e);
            AppError::AuthenticationFailed
        })?;
    
    let response = json!({
        "access_token": auth_response.access_token,
        "refresh_token": auth_response.refresh_token,
        "expires_in": auth_response.expires_in,
    });
    
    // Create new auth cookie
    let cookie = SupabaseAuthService::create_auth_cookie(&auth_response);
    
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(response),
    ))
}

/// User logout endpoint
pub async fn logout(
    State(_state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // Extract access token
    if let Some(token) = SupabaseAuthService::extract_token(&headers) {
        // Initialize Supabase client
        let supabase_config = SupabaseConfig::from_env()
            .map_err(|e| AppError::Configuration(e))?;
        let auth_service = SupabaseAuthService::new(supabase_config);
        
        // Sign out from Supabase
        if let Err(e) = auth_service.sign_out(&token).await {
            tracing::warn!("Supabase sign out failed: {}", e);
        }
    }
    
    // Clear auth cookie
    let clear_cookie = "sb-access-token=; HttpOnly; Secure; SameSite=Lax; Max-Age=0";
    
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, clear_cookie)],
        Json(json!({
            "message": "Logged out successfully",
            "status": "success"
        })),
    ))
}

/// Get current user profile
pub async fn get_profile(
    State(_state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // Extract access token
    let token = SupabaseAuthService::extract_token(&headers)
        .ok_or(AppError::AuthenticationFailed)?;
    
    // Initialize Supabase client
    let supabase_config = SupabaseConfig::from_env()
        .map_err(|e| AppError::Configuration(e))?;
    let auth_service = SupabaseAuthService::new(supabase_config);
    
    // Get user from Supabase
    let user = auth_service
        .get_user(&token)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user profile: {}", e);
            AppError::AuthenticationFailed
        })?;
    
    let profile = json!({
        "id": user.id,
        "email": user.email,
        "email_confirmed_at": user.email_confirmed_at,
        "created_at": user.created_at,
        "updated_at": user.updated_at,
        "user_metadata": user.user_metadata,
        "app_metadata": user.app_metadata,
    });
    
    Ok(Json(profile))
}

/// Update user profile
pub async fn update_profile(
    State(_state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(update_data): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // Extract access token
    let token = SupabaseAuthService::extract_token(&headers)
        .ok_or(AppError::AuthenticationFailed)?;
    
    // Initialize Supabase client
    let supabase_config = SupabaseConfig::from_env()
        .map_err(|e| AppError::Configuration(e))?;
    let auth_service = SupabaseAuthService::new(supabase_config);
    
    // Prepare update input
    let update_input = UpdateUserInput {
        email: update_data.get("email").and_then(|v| v.as_str()).map(String::from),
        password: update_data.get("password").and_then(|v| v.as_str()).map(String::from),
        data: Some(update_data.get("user_metadata").cloned().unwrap_or(json!({}))),
    };
    
    // Update user in Supabase
    let updated_user = auth_service
        .update_user(&token, update_input)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update user profile: {}", e);
            AppError::Validation(format!("Profile update failed: {}", e))
        })?;
    
    Ok(Json(json!({
        "message": "Profile updated successfully",
        "status": "success",
        "user": {
            "id": updated_user.id,
            "email": updated_user.email,
            "user_metadata": updated_user.user_metadata,
        }
    })))
}

/// Request password reset
pub async fn forgot_password(
    State(_state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let email = request
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Validation("Email is required".to_string()))?;
    
    // Initialize Supabase client
    let supabase_config = SupabaseConfig::from_env()
        .map_err(|e| AppError::Configuration(e))?;
    let auth_service = SupabaseAuthService::new(supabase_config);
    
    // Send password reset email
    auth_service
        .reset_password(email)
        .await
        .map_err(|e| {
            tracing::error!("Password reset request failed: {}", e);
            AppError::Validation(format!("Password reset failed: {}", e))
        })?;
    
    Ok(Json(json!({
        "message": "Password reset email sent successfully",
        "status": "success"
    })))
}

/// Verify OTP (for email confirmation or password reset)
pub async fn verify_otp(
    State(_state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    use crate::services::supabase_auth::VerifyOtpInput;
    
    let email = request
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Validation("Email is required".to_string()))?;
    
    let token = request
        .get("token")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Validation("Token is required".to_string()))?;
    
    let otp_type = request
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("signup");
    
    // Initialize Supabase client
    let supabase_config = SupabaseConfig::from_env()
        .map_err(|e| AppError::Configuration(e))?;
    let auth_service = SupabaseAuthService::new(supabase_config);
    
    // Verify OTP
    let verify_input = VerifyOtpInput {
        email: email.to_string(),
        token: token.to_string(),
        r#type: otp_type.to_string(),
    };
    
    let auth_response = auth_service
        .verify_otp(verify_input)
        .await
        .map_err(|e| {
            tracing::error!("OTP verification failed: {}", e);
            AppError::Validation(format!("Verification failed: {}", e))
        })?;
    
    // Create auth cookie
    let cookie = SupabaseAuthService::create_auth_cookie(&auth_response);
    
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(json!({
            "message": "Verification successful",
            "status": "success",
            "access_token": auth_response.access_token,
        })),
    ))
}

/// OAuth provider sign in
pub async fn oauth_signin(
    State(_state): State<AppState>,
    axum::extract::Path(provider): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // Validate provider
    let valid_providers = ["google", "github", "linkedin"];
    if !valid_providers.contains(&provider.as_str()) {
        return Err(AppError::Validation(format!("Invalid OAuth provider: {}", provider)));
    }
    
    // Initialize Supabase client
    let supabase_config = SupabaseConfig::from_env()
        .map_err(|e| AppError::Configuration(e))?;
    let auth_service = SupabaseAuthService::new(supabase_config);
    
    // Get OAuth URL
    let oauth_response = auth_service
        .sign_in_with_oauth(&provider)
        .await
        .map_err(|e| {
            tracing::error!("OAuth sign in failed: {}", e);
            AppError::Validation(format!("OAuth sign in failed: {}", e))
        })?;
    
    Ok(Json(json!({
        "url": oauth_response.url,
        "provider": provider,
    })))
}