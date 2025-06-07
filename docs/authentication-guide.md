# Authentication & RBAC Guide

## Overview

The Loco Platform implements a comprehensive authentication and role-based access control (RBAC) system using JWT tokens and Argon2 password hashing.

## Authentication System

### 1. Local Authentication (JWT-based)

The platform provides local authentication endpoints that work without external dependencies:

#### Register a New User
```bash
POST /api/v1/auth/local/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securePassword123",
  "first_name": "John",
  "last_name": "Doe",
  "phone": "+61412345678",
  "user_type": "Professional"  // Options: Professional, Employer, SuperAdmin
}
```

Response:
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "user_type": "Professional"
  }
}
```

#### Login
```bash
POST /api/v1/auth/local/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securePassword123"
}
```

#### Logout
```bash
POST /api/v1/auth/local/logout
Authorization: Bearer <token>
```

#### Verify Token
```bash
GET /api/v1/auth/local/verify
Authorization: Bearer <token>
```

#### Refresh Token
```bash
POST /api/v1/auth/local/refresh
Authorization: Bearer <token>
```

### 2. Supabase Authentication

The platform also supports Supabase authentication for production environments:

```bash
POST /api/v1/auth/login
POST /api/v1/auth/register
POST /api/v1/auth/logout
POST /api/v1/auth/forgot-password
POST /api/v1/auth/verify-otp
GET /api/v1/auth/oauth/:provider
```

## Role-Based Access Control (RBAC)

### User Roles

1. **SuperAdmin**: Full system access
2. **Admin**: Tenant-level administration
3. **Employer**: Can create and manage job postings
4. **Professional**: Can apply for jobs and manage profile
5. **Guest**: Limited read-only access

### Permissions

The RBAC system defines granular permissions:

```rust
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
```

### Using RBAC Middleware

#### 1. Protecting Routes with Permissions

```rust
use axum::{Router, middleware};
use crate::middleware::rbac::{Permission, check_permission};

let app = Router::new()
    .route("/jobs", 
        post(create_job)
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                check_permission(Permission::CreateJob)
            ))
    );
```

#### 2. Protecting Routes with Roles

```rust
use crate::middleware::rbac::{Role, check_roles};

let app = Router::new()
    .route("/admin/users", 
        get(list_users)
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                check_roles(vec![Role::Admin, Role::SuperAdmin])
            ))
    );
```

#### 3. Combining with JWT Authentication

```rust
use crate::middleware::jwt_auth::jwt_auth_middleware;

let app = Router::new()
    .route("/protected", get(protected_handler))
    .route_layer(middleware::from_fn_with_state(
        state.clone(),
        jwt_auth_middleware
    ));
```

### Accessing User Information in Handlers

```rust
use axum::extract::Extension;
use crate::middleware::rbac::{Role, RequestExt};
use crate::services::auth_service::Claims;

pub async fn protected_handler(
    Extension(claims): Extension<Claims>,
    Extension(role): Extension<Role>,
    request: Request,
) -> Result<impl IntoResponse, AppError> {
    // Access user information
    let user_id = request.user_id();
    let user_email = request.user_email();
    let user_role = request.user_role();
    
    // Use the information
    Ok(Json(json!({
        "user_id": user_id,
        "email": user_email,
        "role": format!("{:?}", user_role),
    })))
}
```

## Security Best Practices

1. **Password Requirements**:
   - Minimum 8 characters
   - Hashed using Argon2 with salt

2. **JWT Token Security**:
   - Tokens expire after 24 hours
   - Stored in HTTP-only cookies
   - Secure and SameSite flags enabled

3. **RBAC Enforcement**:
   - All protected routes require authentication
   - Role and permission checks at middleware level
   - Fail-safe defaults (deny access if uncertain)

## Testing Authentication

### Demo Mode

In demo mode, the authentication system accepts any credentials with minimal validation:
- Email must be provided
- Password must be at least 4 characters
- Returns mock user data

### Production Mode

In production:
- Full validation of all inputs
- Real database storage
- Secure password hashing
- Token validation against JWT secret

## Example Usage

### Creating a Protected Admin Route

```rust
// In your route configuration
.nest("/admin", 
    Router::new()
        .route("/dashboard", get(admin_dashboard))
        .route("/users", get(users::list_users))
        .route("/analytics", get(analytics_handler))
        // Apply admin role check to all routes in this group
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            check_roles(vec![Role::Admin, Role::SuperAdmin])
        ))
        // Require authentication for all admin routes
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            jwt_auth_middleware
        ))
)
```

### Creating a Permission-Based Route

```rust
// Only users with CreateJob permission can access
.route("/jobs", 
    post(jobs::create_job)
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            check_permission(Permission::CreateJob)
        ))
)
```

## Error Responses

### Authentication Errors

- `401 Unauthorized`: Missing or invalid token
- `403 Forbidden`: Insufficient permissions/role

### Example Error Response

```json
{
  "error": "Authentication failed",
  "code": "AUTH_FAILED",
  "status": 401
}
```

## Configuration

Set these environment variables:

```bash
JWT_SECRET=your-secret-key-change-in-production
SUPERADMIN_EMAIL=admin@loco-platform.com
SUPERADMIN_PASSWORD=secure-password
```

## Migration from Other Auth Systems

The platform supports both local JWT and Supabase authentication, allowing gradual migration:

1. Start with local JWT authentication
2. Configure Supabase when ready
3. Both systems can run in parallel
4. Migrate users gradually