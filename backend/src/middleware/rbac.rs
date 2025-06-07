use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
};
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

use crate::{
    services::AuthService,
    AppState, AppError,
};
use shared::types::UserType;

/// Role definitions for the platform
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    SuperAdmin,
    Admin,
    Employer,
    Professional,
    Guest,
}

impl From<&UserType> for Role {
    fn from(user_type: &UserType) -> Self {
        match user_type {
            UserType::SuperAdmin => Role::SuperAdmin,
            UserType::Employer => Role::Employer,
            UserType::Professional => Role::Professional,
        }
    }
}

/// Permission definitions for the platform
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    // Job permissions
    CreateJob,
    EditJob,
    DeleteJob,
    ViewAllJobs,
    ViewOwnJobs,
    
    // User permissions
    ViewAllUsers,
    EditAllUsers,
    DeleteAllUsers,
    ViewOwnProfile,
    EditOwnProfile,
    
    // Application permissions
    ViewAllApplications,
    ViewOwnApplications,
    CreateApplication,
    UpdateApplicationStatus,
    
    // Admin permissions
    ManageTenants,
    ManageSystem,
    ViewAnalytics,
    
    // Messaging permissions
    SendMessage,
    ViewAllMessages,
}

/// Define role-permission mappings
pub fn get_role_permissions(role: &Role) -> HashSet<Permission> {
    use Permission::*;
    
    match role {
        Role::SuperAdmin => {
            // SuperAdmin has all permissions
            vec![
                CreateJob, EditJob, DeleteJob, ViewAllJobs, ViewOwnJobs,
                ViewAllUsers, EditAllUsers, DeleteAllUsers, ViewOwnProfile, EditOwnProfile,
                ViewAllApplications, ViewOwnApplications, CreateApplication, UpdateApplicationStatus,
                ManageTenants, ManageSystem, ViewAnalytics,
                SendMessage, ViewAllMessages,
            ].into_iter().collect()
        },
        Role::Admin => {
            // Admin can manage most things within their tenant
            vec![
                CreateJob, EditJob, DeleteJob, ViewAllJobs, ViewOwnJobs,
                ViewAllUsers, ViewOwnProfile, EditOwnProfile,
                ViewAllApplications, ViewOwnApplications, UpdateApplicationStatus,
                ViewAnalytics,
                SendMessage, ViewAllMessages,
            ].into_iter().collect()
        },
        Role::Employer => {
            // Employers can manage jobs and view applications
            vec![
                CreateJob, EditJob, DeleteJob, ViewOwnJobs,
                ViewOwnProfile, EditOwnProfile,
                ViewOwnApplications, UpdateApplicationStatus,
                SendMessage,
            ].into_iter().collect()
        },
        Role::Professional => {
            // Professionals can apply for jobs and manage their profile
            vec![
                ViewAllJobs,
                ViewOwnProfile, EditOwnProfile,
                ViewOwnApplications, CreateApplication,
                SendMessage,
            ].into_iter().collect()
        },
        Role::Guest => {
            // Guests can only view public jobs
            vec![ViewAllJobs].into_iter().collect()
        },
    }
}

/// Middleware to check if user has required permission
pub async fn require_permission(
    permission: Permission,
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token = AuthService::extract_token_from_header(auth_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Validate token
    let claims = AuthService::validate_token(token, &state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Parse user role from claims
    let user_type = match claims.user_type.as_str() {
        "SuperAdmin" => UserType::SuperAdmin,
        "Employer" => UserType::Employer,
        "Professional" => UserType::Professional,
        _ => return Err(StatusCode::FORBIDDEN),
    };
    
    let role = Role::from(&user_type);
    let permissions = get_role_permissions(&role);
    
    // Check if user has required permission
    if !permissions.contains(&permission) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Add user info to request extensions for use in handlers
    request.extensions_mut().insert(claims);
    request.extensions_mut().insert(role);
    
    Ok(next.run(request).await)
}

/// Middleware to check if user has one of the required roles
pub async fn require_role(
    allowed_roles: Vec<Role>,
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token = AuthService::extract_token_from_header(auth_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Validate token
    let claims = AuthService::validate_token(token, &state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Parse user role from claims
    let user_type = match claims.user_type.as_str() {
        "SuperAdmin" => UserType::SuperAdmin,
        "Employer" => UserType::Employer,
        "Professional" => UserType::Professional,
        _ => return Err(StatusCode::FORBIDDEN),
    };
    
    let role = Role::from(&user_type);
    
    // Check if user has one of the allowed roles
    if !allowed_roles.contains(&role) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Add user info to request extensions for use in handlers
    request.extensions_mut().insert(claims);
    request.extensions_mut().insert(role);
    
    Ok(next.run(request).await)
}

/// Create a permission checking function for use with from_fn_with_state
pub fn check_permission(permission: Permission) -> impl Fn(State<AppState>, Request, Next) -> Pin<Box<dyn Future<Output = Result<Response, StatusCode>> + Send>> + Clone + Send + 'static {
    move |state: State<AppState>, request: Request, next: Next| {
        let permission = permission.clone();
        Box::pin(async move {
            require_permission(permission, state, request, next).await
        })
    }
}

/// Create a role checking function for use with from_fn_with_state  
pub fn check_roles(roles: Vec<Role>) -> impl Fn(State<AppState>, Request, Next) -> Pin<Box<dyn Future<Output = Result<Response, StatusCode>> + Send>> + Clone + Send + 'static {
    move |state: State<AppState>, request: Request, next: Next| {
        let roles = roles.clone();
        Box::pin(async move {
            require_role(roles, state, request, next).await
        })
    }
}

/// Extension trait for extracting user info from request
pub trait RequestExt {
    fn user_id(&self) -> Option<uuid::Uuid>;
    fn user_role(&self) -> Option<&Role>;
    fn user_email(&self) -> Option<&str>;
}

impl RequestExt for Request {
    fn user_id(&self) -> Option<uuid::Uuid> {
        self.extensions()
            .get::<crate::services::auth_service::Claims>()
            .map(|claims| claims.sub)
    }
    
    fn user_role(&self) -> Option<&Role> {
        self.extensions().get::<Role>()
    }
    
    fn user_email(&self) -> Option<&str> {
        self.extensions()
            .get::<crate::services::auth_service::Claims>()
            .map(|claims| claims.email.as_str())
    }
}