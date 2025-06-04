# Technical Context - Loco Platform

## 🦀 Rust Ecosystem Overview

### Core Dependencies
```toml
# Workspace Cargo.toml structure
[workspace]
members = ["frontend", "backend", "shared", "migrations"]
resolver = "2"

[workspace.dependencies]
# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Utilities
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
```

### Frontend Stack (Dioxus + WASM)
```toml
[dependencies]
# Core Dioxus
dioxus = { version = "0.5", features = ["web", "router"] }
dioxus-web = "0.5"

# State management
fermi = "0.4"

# WASM specific
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [
    "Window", "Document", "Element", "HtmlElement",
    "Navigator", "Geolocation", "Storage"
]}
gloo = "0.10"  # Browser API helpers

# HTTP client
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }

# Styling
stylist = { version = "0.13", optional = true }
```

### Backend Stack (Axum + SeaORM)
```toml
[dependencies]
# Web framework
axum = { version = "0.7", features = ["ws", "multipart", "macros"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "compression"] }

# Database
sea-orm = { version = "0.12", features = [
    "sqlx-postgres", "runtime-tokio-rustls", "macros",
    "with-uuid", "with-chrono", "with-json"
]}

# Authentication
jsonwebtoken = "9.2"
argon2 = "0.5"

# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Validation
validator = { version = "0.18", features = ["derive"] }
```

## 🏗️ Project Structure Details

### Frontend Architecture
```
frontend/
├── src/
│   ├── main.rs                 # App entry point, Dioxus launch
│   ├── app.rs                  # Root App component
│   ├── router.rs               # Route definitions
│   │
│   ├── components/             # Reusable UI components
│   │   ├── mod.rs
│   │   ├── layout/
│   │   │   ├── sidebar.rs      # Navigation sidebar
│   │   │   ├── header.rs       # App header
│   │   │   └── footer.rs       # App footer
│   │   ├── common/
│   │   │   ├── button.rs       # Button variants
│   │   │   ├── input.rs        # Form inputs
│   │   │   └── modal.rs        # Modal dialogs
│   │   └── job/
│   │       ├── job_card.rs     # Job listing card
│   │       ├── job_detail.rs   # Full job view
│   │       └── job_form.rs     # Job creation/edit
│   │
│   ├── pages/                  # Route components
│   │   ├── mod.rs
│   │   ├── home.rs             # Landing page
│   │   ├── jobs.rs             # Job listings
│   │   ├── map.rs              # Map view
│   │   ├── profile.rs          # User profile
│   │   └── admin.rs            # Admin panel
│   │
│   ├── hooks/                  # Custom Dioxus hooks
│   │   ├── use_auth.rs         # Authentication state
│   │   ├── use_api.rs          # API client hook
│   │   └── use_debounce.rs     # Debounce helper
│   │
│   ├── services/               # Frontend services
│   │   ├── api.rs              # API client
│   │   ├── auth.rs             # Auth helpers
│   │   └── storage.rs          # LocalStorage wrapper
│   │
│   ├── state/                  # Global state atoms
│   │   ├── mod.rs
│   │   ├── auth.rs             # User/auth state
│   │   ├── jobs.rs             # Job listings cache
│   │   └── ui.rs               # UI preferences
│   │
│   └── utils/                  # Utility functions
│       ├── formatters.rs       # Date, currency formatting
│       ├── validators.rs       # Input validation
│       └── constants.rs        # App constants
```

### Backend Architecture
```
backend/
├── src/
│   ├── main.rs                 # Server entry point
│   ├── config.rs               # Configuration management
│   ├── error.rs                # Error types and handling
│   │
│   ├── handlers/               # Request handlers
│   │   ├── mod.rs
│   │   ├── auth.rs             # Authentication endpoints
│   │   ├── jobs.rs             # Job CRUD endpoints
│   │   ├── users.rs            # User management
│   │   ├── applications.rs     # Job applications
│   │   └── health.rs           # Health checks
│   │
│   ├── models/                 # Domain models
│   │   ├── mod.rs
│   │   ├── job.rs              # Job aggregate
│   │   ├── user.rs             # User aggregate
│   │   └── application.rs      # Application model
│   │
│   ├── services/               # Business logic
│   │   ├── mod.rs
│   │   ├── job_service.rs      # Job operations
│   │   ├── user_service.rs     # User operations
│   │   ├── auth_service.rs     # Auth logic
│   │   └── email_service.rs    # Email notifications
│   │
│   ├── middleware/             # Custom middleware
│   │   ├── auth.rs             # JWT validation
│   │   ├── cors.rs             # CORS setup
│   │   ├── logging.rs          # Request logging
│   │   └── rate_limit.rs       # Rate limiting
│   │
│   ├── db/                     # Database layer
│   │   ├── mod.rs
│   │   ├── connection.rs       # Connection pool
│   │   └── queries/            # Query modules
│   │
│   └── utils/                  # Utilities
│       ├── jwt.rs              # JWT helpers
│       ├── password.rs         # Password hashing
│       └── validators.rs       # Request validation
```

### Shared Types
```
shared/
├── src/
│   ├── lib.rs                  # Public exports
│   ├── types/
│   │   ├── mod.rs
│   │   ├── job.rs              # Job DTOs
│   │   ├── user.rs             # User DTOs
│   │   ├── auth.rs             # Auth types
│   │   └── common.rs           # Common types
│   ├── errors.rs               # Shared error types
│   └── utils.rs                # Shared utilities
```

## 🔧 Build & Development Setup

### Development Commands
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Install development tools
cargo install dioxus-cli
cargo install sea-orm-cli
cargo install cargo-watch
cargo install wasm-pack

# Frontend development
cd frontend
dx serve --hot-reload --port 8080

# Backend development
cd backend
cargo watch -x run

# Run tests
cargo test --all
cargo test --all -- --nocapture  # With println! output

# Linting and formatting
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

### Environment Configuration
```bash
# .env file structure
# Database
DATABASE_URL=postgres://user:pass@localhost/loco_platform
DATABASE_POOL_SIZE=10

# Redis
REDIS_URL=redis://localhost:6379

# Server
HOST=0.0.0.0
PORT=3000
ENVIRONMENT=development

# Security
JWT_SECRET=your-secret-key
JWT_EXPIRY=7d
ARGON2_ITERATIONS=3
ARGON2_MEMORY=65536

# External Services
SMTP_HOST=smtp.sendgrid.net
SMTP_PORT=587
SMTP_USER=apikey
SMTP_PASS=your-api-key

# Feature Flags
ENABLE_SIGNUPS=true
ENABLE_DEMO_MODE=true
```

## 🚀 Performance Optimizations

### WASM Bundle Optimization
```toml
# Cargo.toml release profile
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit
strip = true        # Strip symbols

# Additional wasm-opt pass
# wasm-opt -Oz -o output.wasm input.wasm
```

### Database Query Optimization
```rust
// Efficient query with joins and selection
let jobs_with_company = Job::find()
    .find_also_related(Company)
    .select_only()
    .column(job::Column::Id)
    .column(job::Column::Title)
    .column(job::Column::Salary)
    .column(company::Column::Name)
    .filter(job::Column::Status.eq("active"))
    .limit(50)
    .all(db)
    .await?;

// Use prepared statements
let stmt = Statement::from_sql_and_values(
    DbBackend::Postgres,
    r#"SELECT * FROM jobs WHERE postcode = $1 AND status = $2"#,
    vec![postcode.into(), "active".into()],
);
```

### Caching Strategy
```rust
// Redis caching layer
use redis::AsyncCommands;

pub async fn get_cached_jobs(
    redis: &redis::Client,
    key: &str,
) -> Result<Option<Vec<Job>>> {
    let mut conn = redis.get_async_connection().await?;
    let cached: Option<String> = conn.get(key).await?;
    
    Ok(cached.and_then(|data| serde_json::from_str(&data).ok()))
}

// Cache with expiration
pub async fn cache_jobs(
    redis: &redis::Client,
    key: &str,
    jobs: &[Job],
    ttl: usize,
) -> Result<()> {
    let mut conn = redis.get_async_connection().await?;
    let data = serde_json::to_string(jobs)?;
    conn.setex(key, ttl, data).await?;
    Ok(())
}
```

## 🔒 Security Implementation

### JWT Token Structure
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,              // User ID
    pub email: String,
    pub role: UserRole,
    pub exp: usize,             // Expiration
    pub iat: usize,             // Issued at
    pub jti: Uuid,              // Token ID for revocation
}

// Token generation
pub fn generate_token(user: &User) -> Result<String> {
    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        role: user.role,
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
        jti: Uuid::new_v4(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}
```

### Input Validation
```rust
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateJobRequest {
    #[validate(length(min = 3, max = 200))]
    pub title: String,
    
    #[validate(length(min = 10, max = 5000))]
    pub description: String,
    
    #[validate(custom = "validate_postcode")]
    pub postcode: String,
    
    #[validate(range(min = 0, max = 1000000))]
    pub salary: Option<i32>,
}

fn validate_postcode(postcode: &str) -> Result<(), ValidationError> {
    // Australian postcode validation
    if postcode.len() == 4 && postcode.chars().all(|c| c.is_numeric()) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_postcode"))
    }
}
```

## 📊 Monitoring & Observability

### Structured Logging
```rust
use tracing::{info, error, instrument};

#[instrument(skip(db))]
pub async fn create_job(
    db: &DatabaseConnection,
    input: CreateJobRequest,
) -> Result<Job> {
    info!("Creating new job listing");
    
    let job = job::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(input.title),
        description: Set(input.description),
        created_at: Set(Utc::now()),
        ..Default::default()
    };
    
    match job.insert(db).await {
        Ok(job) => {
            info!(job_id = %job.id, "Job created successfully");
            Ok(job)
        }
        Err(e) => {
            error!(error = %e, "Failed to create job");
            Err(e.into())
        }
    }
}
```

### Health Checks
```rust
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>> {
    // Check database
    let db_healthy = sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();
    
    // Check Redis
    let redis_healthy = state.redis
        .get_async_connection::<()>()
        .await
        .is_ok();
    
    Ok(Json(HealthResponse {
        status: if db_healthy && redis_healthy { "healthy" } else { "unhealthy" },
        database: db_healthy,
        cache: redis_healthy,
        version: env!("CARGO_PKG_VERSION"),
    }))
}
```

## 🌐 API Design

### RESTful Endpoints
```
# Authentication
POST   /api/auth/register
POST   /api/auth/login
POST   /api/auth/refresh
POST   /api/auth/logout

# Jobs
GET    /api/jobs                    # List with filters
POST   /api/jobs                    # Create new job
GET    /api/jobs/:id                # Get job details
PUT    /api/jobs/:id                # Update job
DELETE /api/jobs/:id                # Delete job
GET    /api/jobs/search             # Advanced search
GET    /api/jobs/nearby/:postcode   # Geographic search

# Applications
POST   /api/jobs/:id/apply          # Submit application
GET    /api/applications            # List user's applications
GET    /api/applications/:id        # Application details
PUT    /api/applications/:id        # Update application
DELETE /api/applications/:id        # Withdraw application

# Users
GET    /api/users/profile           # Current user profile
PUT    /api/users/profile           # Update profile
POST   /api/users/avatar            # Upload avatar
GET    /api/users/:id               # Public profile

# Real-time
WS     /ws                          # WebSocket connection
```

### Error Response Format
```rust
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

impl From<AppError> for ErrorResponse {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Validation(errors) => ErrorResponse {
                error: "Validation Error".to_string(),
                message: "Invalid input provided".to_string(),
                code: "VALIDATION_ERROR".to_string(),
                details: Some(json!(errors)),
            },
            AppError::NotFound => ErrorResponse {
                error: "Not Found".to_string(),
                message: "Resource not found".to_string(),
                code: "NOT_FOUND".to_string(),
                details: None,
            },
            // ... other error mappings
        }
    }
}
```

## 🔄 Deployment Pipeline

### Docker Configuration
```dockerfile
# Multi-stage build for optimal size
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build dependencies first for caching
RUN cargo build --release --workspace

# Final stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/
COPY --from=builder /app/frontend/dist /usr/share/loco/static

EXPOSE 3000
CMD ["backend"]
```

### CI/CD with GitHub Actions
```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2
        
      - name: Run tests
        run: cargo test --all-features --workspace
        
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
        
      - name: Check formatting
        run: cargo fmt --all -- --check

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build Docker image
        run: docker build -t loco-platform:${{ github.sha }} .
        
      - name: Push to registry
        if: github.ref == 'refs/heads/main'
        run: |
          echo ${{ secrets.DOCKER_PASSWORD }} | docker login -u ${{ secrets.DOCKER_USERNAME }} --password-stdin
          docker push loco-platform:${{ github.sha }}
```

---

**Last Updated**: January 2025
**Version**: 1.0
**Tech Stack Version**: Rust 1.75, Dioxus 0.5, Axum 0.7