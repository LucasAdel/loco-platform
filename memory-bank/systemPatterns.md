# System Patterns - Loco Platform

## 🏗️ Architecture Patterns

### Cargo Workspace Structure
```
loco-platform/
├── Cargo.toml (workspace)
├── frontend/ (Leptos + WebAssembly) [MIGRATED 2025-01-06]
├── backend/ (Axum + SeaORM)
├── shared/ (Common types)
└── app/ (Minimal app crate for cross-compilation)
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

### Frontend Patterns (Leptos) [MIGRATED 2025-01-06]

#### Component Structure
```rust
#[component]
fn ComponentName(props: Props) -> impl IntoView {
    let (state, set_state) = create_signal(initial_value);
    
    view! {
        <div class="component-container">
            // HTML-like syntax with reactive signals
        </div>
    }
}
```

#### State Management with Leptos Signals
```rust
// Reactive signals for global state
let (jobs, set_jobs) = create_signal(Vec::<Job>::new());
let (search_filters, set_search_filters) = create_signal(JobFilters::default());

// Signal composition and derived state
let filtered_jobs = create_memo(move |_| {
    jobs.with(|j| filter_jobs(j, &search_filters.get()))
});
```

#### Reactive Patterns
```rust
// Effect for side effects
create_effect(move |_| {
    let current_filters = search_filters.get();
    // Automatically runs when search_filters changes
});

// Resource for async data loading
let jobs_resource = create_resource(
    move || search_filters.get(),
    |filters| async move {
        fetch_jobs_from_api(filters).await
    },
);
```

#### API Client Pattern (Leptos)
```rust
// Leptos-compatible API client with gloo-net
pub struct ApiClient {
    client: gloo_net::http::Request,
    base_url: String,
}

impl ApiClient {
    pub async fn fetch_jobs(&self) -> Result<Vec<Job>, ApiError> {
        let response = gloo_net::http::Request::get(&format!("{}/api/jobs", self.base_url))
            .send()
            .await?
            .json::<Vec<Job>>()
            .await?;
        Ok(response)
    }
}

// Server function pattern for SSR
#[server(FetchJobs, "/api")]
pub async fn fetch_jobs(filters: JobFilters) -> Result<Vec<Job>, ServerFnError> {
    // Server-side implementation for SSR/hydration
}
```

### Backend Patterns (Axum) [IMPLEMENTED]

#### Handler Structure
```rust
#[axum::debug_handler]
async fn handler_name(
    State(state): State<AppState>,
    Json(payload): Json<RequestType>,
) -> Result<Json<ResponseType>, AppError> {
    // Validated implementation with proper error handling
    let result = state.service.process(payload).await?;
    Ok(Json(result))
}
```

#### Service Layer Pattern
```rust
// Business logic separation
pub struct JobService {
    repository: JobRepository,
}

impl JobService {
    pub async fn create_job(&self, job_data: CreateJobInput) -> Result<Job, ServiceError> {
        // Business logic and validation
        self.repository.save(job_data).await
    }
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

### Database Patterns (SeaORM) [MODELS IMPLEMENTED]

#### Entity Definition Pattern
```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "jobs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub company_name: String,
    pub location: String,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub job_type: JobType,
    pub experience_level: ExperienceLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Australian-specific fields
    pub postcode: String,
    pub state: AustralianState,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::application::Entity")]
    Applications,
}
```

#### Repository Pattern
```rust
pub struct JobRepository {
    db: DatabaseConnection,
}

impl JobRepository {
    pub async fn find_by_filters(&self, filters: JobFilters) -> Result<Vec<job::Model>, DbErr> {
        let mut query = Job::find();
        
        if let Some(location) = filters.location {
            query = query.filter(job::Column::Location.contains(&location));
        }
        
        query.all(&self.db).await
    }
}
```

## 🎨 Design Patterns

### Modern Design System (2025)
- **Advanced Glass Morphism** with multi-layered backdrop effects
- **Beautiful Modern Typography** using Inter font family
- **Sophisticated Color System** with CSS custom properties
- **Professional Sidebar Navigation** with collapse/expand functionality
- **Micro-interactions** with smooth, delightful animations
- **Heroicons Integration** alongside Font Awesome for rich iconography
- **Consistent Design Language** across all static HTML pages

### CSS Architecture
```css
/* Glass morphism with enhanced visibility */
.sidebar {
  background: rgba(255, 255, 255, 0.25);
  backdrop-filter: blur(20px);
  border-right: 1px solid rgba(255, 255, 255, 0.4);
  box-shadow: 4px 0 30px rgba(0, 0, 0, 0.2);
}

/* Fallback for browsers without backdrop-filter */
@supports not (backdrop-filter: blur(20px)) {
  .sidebar {
    background: rgba(30, 41, 59, 0.95);
  }
}
```

### Modern Color Palette
```css
:root {
  /* Primary Blues (Modern gradient spectrum) */
  --primary-50: #f0f9ff;
  --primary-100: #e0f2fe;
  --primary-500: #0ea5e9;
  --primary-900: #0c4a6e;
  
  /* Neutral Grays with warm undertones */
  --gray-50: #fafaf9;
  --gray-900: #1c1917;
  
  /* Glass morphism variables */
  --glass-bg: rgba(255, 255, 255, 0.1);
  --glass-border: rgba(255, 255, 255, 0.2);
  --glass-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}
```

### Component Design Patterns
- **Glass Cards** with hover lift effects and shimmer animations
- **Floating Action Buttons** with ripple effects
- **Collapsible Sidebar** with smooth transitions and tooltips
- **Modern Input Fields** with glass styling (glass-input class)
- **Advanced Loading States** with pulse animations
- **Interactive Dashboard** with real-time counters
- **Glass Buttons** (glass-button, glass-button-secondary) with consistent styling
- **Modern Card Components** (.modern-card) for job listings and content
- **Glass Tables** for data display with modern styling

### Animation System
```css
/* Modern easing curves */
--ease-bounce: cubic-bezier(0.68, -0.55, 0.265, 1.55);
--ease-elastic: cubic-bezier(0.25, 0.46, 0.45, 0.94);

/* Micro-interactions */
.bounce-in { animation: bounceIn 500ms var(--ease-bounce); }
.fade-in-up { animation: fadeInUp 500ms var(--ease-out); }
.slide-in-right { animation: slideInRight 500ms var(--ease-out); }
```

### Mobile-First Responsive Design
- **Collapsible sidebar** transforms to overlay on mobile
- **Touch-optimized** with 48px minimum touch targets
- **Smooth scrolling** with momentum on mobile devices
- **Progressive enhancement** for older browsers

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

### Authentication [MIDDLEWARE IMPLEMENTED]
- **JWT tokens** with proper expiration and validation
- **Role-based access control** (RBAC) with user roles
- **Secure password hashing** with Argon2
- **Middleware integration** with Axum request processing
- **Session management** with secure token handling

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

### Testing Strategy [FRAMEWORK READY]
- **Unit tests** for business logic (cargo test)
- **Integration tests** for API endpoints (axum-test)
- **Component tests** for UI (leptos-testing)
- **Property-based tests** with proptest
- **E2E tests** with browser automation (Playwright configured)
- **Cross-platform testing** for web and desktop targets

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

### Monitoring [HEALTH CHECKS IMPLEMENTED]
- **Health checks** for services (implemented in backend)
- **Structured logging** with tracing throughout application
- **Error tracking** with proper context preservation
- **Performance monitoring** ready for Prometheus integration
- **Real-time metrics** for job processing and user activity

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

## 🔮 Cross-Platform Patterns [NEW]

### Platform Abstraction
```rust
// Platform detection and adaptation
#[cfg(target_arch = "wasm32")]
mod web_platform {
    // Web-specific implementations
}

#[cfg(not(target_arch = "wasm32"))]
mod desktop_platform {
    // Desktop-specific implementations (Tauri)
}
```

### Build Target Patterns
- **Conditional compilation** for web vs desktop features
- **Shared component library** with platform-specific adaptations
- **Unified state management** across platforms
- **Platform-specific optimisations** (bundle size vs performance)

### Future Patterns

#### Scalability Preparation
- **Microservices readiness** with clear service boundaries
- **Event-driven architecture** with async message handling
- **Horizontal scaling** with stateless design patterns
- **Cross-platform deployment** strategies

#### Feature Toggles
- **Gradual rollouts** with feature flags
- **A/B testing** infrastructure for UX improvements
- **Platform-specific features** (desktop notifications, web push)
- **Backwards compatibility** maintenance across versions