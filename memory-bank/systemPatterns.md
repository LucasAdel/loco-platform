# System Patterns - Loco Platform

## 🏗️ Architecture Patterns

### Cargo Workspace Structure
```
loco-platform/
├── Cargo.toml (workspace)
├── frontend/ (Dioxus + WebAssembly)
├── backend/ (Axum + SeaORM)
└── shared/ (Common types)
```

### Type System Patterns

#### NewType Wrappers (Implemented)
```rust
// Australian-specific types with validation
pub struct Postcode(String);
pub struct PhoneNumber(String);
pub struct JobId(Uuid);
```

#### Error Handling Pattern
```rust
// Comprehensive error types with thiserror
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("Validation error: {0}")]
    Validation(String),
}
```

### Frontend Patterns (Dioxus)

#### Component Structure
```rust
#[component]
fn ComponentName(props: Props) -> Element {
    let state = use_signal(|| initial_value);
    
    rsx! {
        div { class: "component-container",
            // JSX-like syntax
        }
    }
}
```

#### State Management with Fermi
```rust
// Global atoms for shared state
static JOBS: Atom<Vec<Job>> = Atom(|_| vec![]);
static SEARCH_FILTERS: Atom<JobFilters> = Atom(|_| JobFilters::default());
```

#### API Client Pattern
```rust
// Centralized API client with error handling
pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub async fn fetch_jobs(&self) -> Result<Vec<Job>, ApiError> {
        // Implementation with proper error handling
    }
}
```

### Backend Patterns (Axum)

#### Handler Structure (To Implement)
```rust
async fn handler_name(
    State(state): State<AppState>,
    Json(payload): Json<RequestType>,
) -> Result<Json<ResponseType>, AppError> {
    // Implementation
}
```

#### Middleware Pattern
```rust
pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Authentication logic
}
```

### Database Patterns (SeaORM - To Implement)

#### Entity Definition Pattern
```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "jobs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub description: String,
    // Australian-specific fields
    pub postcode: Postcode,
}
```

## 🎨 Design Patterns

### CSS Architecture
- **Glass morphism effects** with backdrop-blur
- **Australian colour palette** (blues, greens, gold)
- **Accessibility-first** design with proper contrast
- **Mobile-first responsive** design

### Component Design
- **Consistent spacing** using Tailwind utilities
- **Hover states** with smooth transitions
- **Focus management** for keyboard navigation
- **Loading states** with proper feedback

## 🔄 Data Flow Patterns

### Frontend Data Flow
```
User Interaction → Component State → API Call → Global State Update → UI Re-render
```

### Error Handling Flow
```
Error Occurs → Type-safe Error Enum → User-friendly Message → Recovery Action
```

## 🛡️ Security Patterns

### Input Validation
- **Type-level validation** using Rust's type system
- **Serde validation** for API boundaries
- **Australian-specific validation** (postcodes, phones)

### Authentication (To Implement)
- **JWT tokens** with proper expiration
- **Role-based access control** (RBAC)
- **Secure password hashing** with Argon2

## 📱 Mobile-First Patterns

### Responsive Design
- **Breakpoint strategy** with Tailwind classes
- **Touch-friendly targets** (48px minimum)
- **Smooth scrolling** with momentum
- **Safe area handling** for mobile devices

### Progressive Web App
- **Service worker** for offline functionality
- **App manifest** for installation
- **Push notifications** for job alerts

## 🔧 Development Patterns

### Code Organisation
- **Feature-based modules** for scalability
- **Shared utilities** in common crate
- **Type-driven development** with Rust

### Testing Strategy (To Implement)
- **Unit tests** for business logic
- **Integration tests** for API endpoints
- **Component tests** for UI
- **Property-based tests** with proptest

## 🚀 Performance Patterns

### WASM Optimisation
- **Code splitting** at component level
- **Lazy loading** for heavy components
- **Bundle size optimisation** with tree shaking

### Database Optimisation
- **Query optimisation** with proper indexes
- **Connection pooling** with deadpool
- **Caching strategies** with Redis

## 📊 Observability Patterns

### Logging
- **Structured logging** with tracing
- **Request tracing** across service boundaries
- **Error context** preservation

### Monitoring (To Implement)
- **Health checks** for services
- **Metrics collection** with Prometheus
- **Alerting** for critical issues

## 🌏 Australian Localisation Patterns

### Data Formatting
- **Currency formatting** for AUD
- **Date/time formatting** for Australian timezones
- **Address validation** for Australian postcodes
- **Phone number formatting** for Australian formats

### Business Logic
- **Pharmacy-specific** job categories
- **Australian healthcare** compliance
- **Aboriginal and Torres Strait Islander** place names support

## 🔮 Future Patterns

### Scalability Preparation
- **Microservices readiness** with clear boundaries
- **Event-driven architecture** with message queues
- **Horizontal scaling** considerations

### Feature Toggles
- **Gradual rollouts** with feature flags
- **A/B testing** infrastructure
- **Backwards compatibility** maintenance