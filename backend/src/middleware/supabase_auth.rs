use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use shared::supabase::TenantClaims;
use uuid::Uuid;

/// Supabase JWT verification middleware
pub async fn supabase_auth_middleware(
    State(app_state): State<crate::AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extract bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Decode and validate JWT
    let claims = verify_supabase_jwt(token, &app_state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Verify Supabase JWT token
fn verify_supabase_jwt(token: &str, secret: &str) -> Result<TenantClaims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<TenantClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

/// Extract tenant context from request
pub fn extract_tenant_context(request: &Request) -> Option<TenantContext> {
    request.extensions().get::<TenantClaims>().map(|claims| {
        TenantContext {
            tenant_id: claims.tenant_id,
            user_id: claims.sub,
            role: claims.role.clone(),
            permissions: claims.permissions.clone(),
        }
    })
}

#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub permissions: Vec<String>,
}

/// Require specific permission
pub async fn require_permission(
    permission: &str,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let context = extract_tenant_context(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    if !context.permissions.contains(&permission.to_string()) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(next.run(request).await)
}

/// Require specific role
pub async fn require_role(
    role: &str,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let context = extract_tenant_context(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Check role hierarchy
    let allowed = match (role, context.role.as_str()) {
        ("Owner", "Owner") => true,
        ("Admin", "Owner" | "Admin") => true,
        ("Manager", "Owner" | "Admin" | "Manager") => true,
        ("Member", _) => true,
        _ => false,
    };
    
    if !allowed {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(next.run(request).await)
}

/// Apply tenant filter to database queries
#[macro_export]
macro_rules! with_tenant_filter {
    ($query:expr, $tenant_id:expr) => {
        $query.filter(sea_orm::ColumnTrait::eq(
            <_ as sea_orm::EntityTrait>::Column::TenantId,
            $tenant_id
        ))
    };
}