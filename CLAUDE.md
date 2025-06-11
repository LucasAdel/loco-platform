# CLAUDE.md - Loco Platform Project Instructions

## 🚨 CRITICAL: Start of Every Session Protocol

1. **Read this CLAUDE.md file completely**
2. **Check checklist.md for current tasks and priorities**
3. **Read all Memory Bank files** (./memory-bank/*)
4. **Review recent changes in activeContext.md**
5. **Identify next tasks from checklist.md**

## 🔄 MANDATORY: Continuous Checklist.md Management

### ALWAYS UPDATE CHECKLIST.MD WITH EVERY CHANGE
- **After implementing ANY feature**: Mark items as completed with exact date
- **When discovering new requirements**: Add granular tasks to appropriate sections
- **When user requests new functionality**: Create detailed checklist items immediately
- **When fixing bugs**: Add technical debt items to prevent regression
- **When optimizing performance**: Add performance monitoring tasks
- **When adding dependencies**: Add maintenance and update tasks

### CHECKLIST.MD EXPANSION PROTOCOL
1. **Never reduce the checklist** - only add and complete items
2. **Break down large features** into granular, actionable tasks
3. **Add completion dates** for every finished item
4. **Create new phases** when logical groupings emerge
5. **Add implementation notes** for complex completed items
6. **Cross-reference related features** to prevent duplication
7. **Update completion statistics** after significant progress

### FEATURE ADDITION WORKFLOW
```
User Request → Analyze Requirements → Add to Checklist.md → 
Plan Implementation → Execute → Mark Complete → Add Related Features
```

**COMMAND**: After every significant change, run this mental checklist:
- [ ] Is checklist.md updated with new tasks?
- [ ] Are completed items marked with today's date?
- [ ] Have I added any related features that were discovered?
- [ ] Are the completion statistics updated?
- [ ] Have I preserved all existing checklist content?

## 🚨 MANDATORY MEMORY BANK PROTECTION RULE

### ⛔ ABSOLUTE PROHIBITION: DELETION OR REMOVAL OF MEMORY BANK CONTENT

**CRITICAL PROTECTION PROTOCOL - THIS RULE CANNOT BE OVERRIDDEN:**

#### 🔒 **FORBIDDEN ACTIONS**
- ❌ **DELETION** of any existing memory bank content
- ❌ **REMOVAL** of any historical information
- ❌ **OVERWRITING** of existing documentation
- ❌ **TRUNCATION** of any sections or entries
- ❌ **REPLACEMENT** of existing content with new content
- ❌ **RESET** or **REVERT** operations that affect memory bank files
- ❌ **ARCHIVAL** that removes content from active memory bank

#### ✅ **ONLY PERMITTED ACTIONS**
- ✅ **ADDITION** of new content at the end
- ✅ **APPENDING** new lessons learned
- ✅ **SUPPLEMENTING** existing sections with additional information
- ✅ **EXPANDING** documentation with more detail
- ✅ **ENHANCEMENT** of existing content (without removing original)

#### 🛡️ **PROTECTION ENFORCEMENT**
- **Before ANY memory bank modification**: Verify operation is ADDITION ONLY
- **Before git operations**: Explicitly exclude memory-bank/ from resets
- **When restoring code**: PRESERVE memory bank separately
- **When making changes**: ALWAYS add new content, never replace

#### 📋 **MANDATORY PRE-MODIFICATION CHECKLIST**
Before touching memory bank files, confirm:
- [ ] Am I ONLY adding new content?
- [ ] Am I preserving ALL existing content?
- [ ] Am I avoiding ANY deletion operations?
- [ ] Have I excluded memory-bank/ from git resets?
- [ ] Will this preserve the complete historical record?

**VIOLATION CONSEQUENCE**: Immediate restoration from git history required

**RATIONALE**: Memory bank contains irreplaceable project knowledge spanning months of development, including critical infrastructure documentation, security implementations, and decision trails that cannot be recreated.

---

## Project Overview

**Loco Platform** - A comprehensive Rust web application built with Dioxus, Axum, SeaORM, and modern Rust ecosystem tools.

### Current Status (January 2025)
- 🔄 **Development Phase**
- ✅ **Rust Best Practices**
- ✅ **Type Safety with Zero-Cost Abstractions**
- ✅ **Dioxus Web + Desktop Compatible**
- ✅ **Professional Rust Project Structure**

## Key Files & References

### Essential Files (Always Check These)
1. **`./checklist.md`** - Master task list and project roadmap
2. **`./memory-bank/activeContext.md`** - Current state and recent changes
3. **`./memory-bank/systemPatterns.md`** - Architecture and patterns
4. **`./memory-bank/progress.md`** - Completed features and todo items
5. **`Cargo.toml`** - Project dependencies and workspace configuration

### Directory Structure
```
loco-platform/
├── Cargo.toml                 # Workspace configuration
├── frontend/                  # Dioxus frontend
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── components/        # Dioxus components
│   │   ├── pages/            # Page components
│   │   ├── hooks/            # Custom Dioxus hooks
│   │   ├── services/         # API client services
│   │   └── utils/            # Utility functions
├── backend/                   # Axum backend
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── handlers/         # Route handlers
│   │   ├── models/           # Data models
│   │   ├── services/         # Business logic
│   │   └── middleware/       # Custom middleware
├── shared/                    # Shared types and utilities
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── types/            # Shared data types
│       └── utils/            # Shared utilities
├── docs/                      # Documentation
├── tests/                     # Integration tests
├── migrations/                # Database migrations
├── memory-bank/               # Project memory
├── CLAUDE.md                  # This file
└── checklist.md              # Master task list
```

## Technology Stack

### Frontend
- **Dioxus** with WebAssembly
- **Dioxus Router** for navigation
- **Fermi** for state management
- **Stylist** or **TailwindCSS** for styling
- **Web-sys** for browser APIs

### Backend
- **Axum** web framework
- **SeaORM** for database operations
- **Tokio** async runtime
- **Tower** middleware
- **PostgreSQL** database

### Shared Tools
- **Serde** for serialization
- **Chrono** for date/time handling
- **UUID** for identifiers
- **Thiserror** for error handling
- **Tracing** for logging

### Testing & Quality
- **Cargo test** for unit tests
- **Clippy** for linting
- **Rustfmt** for formatting
- **Cargo audit** for security

## Development Guidelines

### 1. Australian English Only
Use Australian English in ALL:
- Code comments
- Documentation
- UI text
- Variable names (e.g., `colour` not `color`)

### 2. Feature Preservation
**NEVER delete or break existing features**
- Always test before and after changes
- Maintain backward compatibility
- Document any breaking changes
- Use feature flags for major changes

### 3. Code Quality Standards
```rust
// ✅ Good: Clear types, error handling
pub async fn fetch_jobs(filters: JobFilters) -> Result<Vec<Job>, AppError> {
    let jobs = Job::find()
        .filter(job::Column::Status.eq("active"))
        .all(&db)
        .await
        .map_err(AppError::Database)?;

    Ok(jobs)
}

// ❌ Bad: No error handling, unclear return type
pub async fn fetch_jobs(filters: JobFilters) -> Vec<Job> {
    Job::find().all(&db).await.unwrap()
}
```

### 4. Component Patterns

#### Dioxus Component Structure
```rust
use dioxus::prelude::*;

#[component]
pub fn JobCard(job: Job) -> Element {
    let theme = use_context::<Theme>();

    rsx! {
        div {
            class: "job-card bg-white/70 backdrop-blur-xl border rounded-xl p-4",
            h3 { class: "text-lg font-semibold", "{job.title}" }
            p { class: "text-gray-600", "{job.description}" }
            button {
                class: "bg-blue-500 hover:bg-blue-600 rounded-lg px-4 py-2 text-white",
                onclick: move |_| {
                    // Handle job application
                },
                "Apply Now"
            }
        }
    }
}
```

#### Error Handling Pattern
```rust
use dioxus::prelude::*;

#[component]
pub fn ErrorBoundary(children: Element) -> Element {
    let error = use_signal(|| None::<String>);

    if let Some(err) = error() {
        rsx! {
            div { class: "error-boundary bg-red-50 border border-red-200 rounded-lg p-4",
                h3 { "Something went wrong" }
                p { "{err}" }
                button {
                    onclick: move |_| error.set(None),
                    "Try Again"
                }
            }
        }
    } else {
        children
    }
}
```

#### Loading States
```rust
#[component]
pub fn LoadingWrapper(loading: bool, error: Option<String>, children: Element) -> Element {
    match (loading, error) {
        (true, _) => rsx! {
            div { class: "loading-spinner animate-spin", "Loading..." }
        },
        (false, Some(err)) => rsx! {
            div { class: "error-message text-red-600", "{err}" }
        },
        (false, None) => children,
    }
}
```

## Development Workflow

### Starting Development
```bash
# 1. Install Rust and tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install dioxus-cli
cargo install sea-orm-cli

# 2. Set up environment
cp .env.example .env

# 3. Start development server
dx serve --hot-reload

# 4. Run backend (separate terminal)
cargo run -p backend

# 5. Run tests
cargo test --all
cargo clippy --all-targets
```

### Making Changes
1. Check `checklist.md` for next task
2. Create feature branch
3. Implement with tests
4. Update documentation
5. Update `checklist.md` status
6. Commit with descriptive message

### Commit Format
```
type(scope): description

- Updated checklist.md item X
- Fixed/Added/Changed specific details
- Related to issue #Y
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Critical Patterns & Conventions

### 1. Database Patterns
```rust
use sea_orm::*;

// Always use Result types for database operations
pub async fn create_job(db: &DatabaseConnection, job_data: CreateJobInput) -> Result<job::Model, DbErr> {
    let job = job::ActiveModel {
        title: Set(job_data.title),
        description: Set(job_data.description),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    job.insert(db).await
}
```

### 2. Authentication Flow
```rust
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate JWT token
    let token = validate_jwt(auth_header)?;
    request.extensions_mut().insert(token);

    Ok(next.run(request).await)
}
```

### 3. API Client Pattern
```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub async fn fetch_jobs(&self, filters: JobFilters) -> Result<Vec<Job>, reqwest::Error> {
        self.client
            .get(&format!("{}/api/jobs", self.base_url))
            .query(&filters)
            .send()
            .await?
            .json()
            .await
    }
}
```

### 4. Mock Data Fallback
```rust
// Always provide fallback for development
pub fn get_sample_jobs() -> Vec<Job> {
    vec![
        Job {
            id: Uuid::new_v4(),
            title: "Senior Pharmacist".to_string(),
            description: "Leading pharmacy role in Sydney CBD".to_string(),
            location: "Sydney, NSW".to_string(),
            ..Default::default()
        },
        // More sample data...
    ]
}
```

## Testing Requirements

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_job() {
        let db = setup_test_db().await;
        let job_data = CreateJobInput {
            title: "Test Job".to_string(),
            description: "Test description".to_string(),
        };

        let result = create_job(&db, job_data).await;
        assert!(result.is_ok());

        let job = result.unwrap();
        assert_eq!(job.title, "Test Job");
    }
}
```

### Integration Testing
```rust
use axum_test::TestServer;

#[tokio::test]
async fn test_job_api() {
    let app = create_test_app().await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/jobs")
        .json(&serde_json::json!({
            "title": "Test Job",
            "description": "Test description"
        }))
        .await;

    assert_eq!(response.status_code(), 201);
}
```

### Manual Testing Checklist
- [ ] All pages load without panics
- [ ] Authentication flow works
- [ ] Database operations complete successfully
- [ ] API endpoints return correct data
- [ ] Frontend components render properly
- [ ] No clippy warnings

## Security Considerations

### Environment Variables
- **Never commit** `.env` files
- Use `.env.example` as template
- Store secrets securely
- Use environment-specific configs

### Authentication
- Always validate user permissions
- Use secure password hashing (argon2)
- Implement proper session management
- Sanitise user inputs

## Known Issues & Solutions

### WASM Compatibility
- Use `wasm-bindgen` for browser APIs
- Check feature flags for WASM-specific code
- Use `console_error_panic_hook` for debugging

### Database Migrations
- Use SeaORM migrations
- Test migrations thoroughly
- Backup before major changes

### Cross-Origin Issues
- Configure CORS properly in Axum
- Use appropriate headers for API calls

## Deployment

### Development Build
```bash
# Build frontend
dx build

# Build backend
cargo build --release -p backend

# Run migrations
sea-orm-cli migrate up
```

### Production Build
```bash
# Optimised build
dx build --release
cargo build --release

# Docker deployment
docker build -t loco-platform .
docker run -p 8080:8080 loco-platform
```

## Memory Bank Usage

### Reading Memory Bank
```rust
// At session start
1. Read projectbrief.md for overview
2. Read activeContext.md for current state
3. Read systemPatterns.md for patterns
4. Read progress.md for task status
```

### Updating Memory Bank
```rust
// After significant changes
1. Update activeContext.md with changes
2. Update progress.md with completed tasks
3. Add new patterns to systemPatterns.md
4. Document decisions in appropriate files
```

## Quick Commands

### Development
- `dx serve` - Start Dioxus dev server with hot reload
- `cargo run -p backend` - Start backend server
- `cargo test --all` - Run all tests
- `cargo clippy --all-targets` - Run linter

### Database
- `sea-orm-cli migrate up` - Run migrations
- `sea-orm-cli generate entity` - Generate entities
- Check database logs for query optimisation

### Debugging
- Use `tracing` for structured logging
- `cargo expand` to see macro expansions
- `cargo flamegraph` for performance profiling

## Contact & Support

### Project Resources
- **Documentation**: `/docs/`
- **Memory Bank**: `/memory-bank/`
- **Examples**: `/examples/`
- **Tests**: `/tests/`

### Development Tools
- **Rust Analyzer** for IDE support
- **Cargo Watch** for auto-rebuilding
- **Dioxus DevTools** for component inspection

## Final Reminders

1. **Always preserve existing functionality**
2. **Test thoroughly before committing**
3. **Update documentation as you go**
4. **Check checklist.md for priorities**
5. **Use Australian English**
6. **Think deeply, implement carefully**
7. **Maintain Rust best practices**
8. **Follow established patterns**
9. **Leverage Rust's type system for safety**
10. **Use `Result<T, E>` for error handling**

---

**Remember**: This project emphasises Rust's safety, performance, and maintainability principles. Every change should leverage Rust's type system and ownership model. When in doubt, refer to the Memory Bank files for context and patterns.

**Current Focus**: Check `checklist.md` and `memory-bank/activeContext.md` for immediate priorities.

## Memory Bank Preservation Protocol

  ### CRITICAL INSTRUCTION: Never Delete Memory Bank Content

  **MANDATORY PRESERVATION RULE**: When updating memory bank
  files, you MUST preserve all historical information. Never
  delete or remove any lines from memory bank files.

  ### Memory Bank Archive System

  When memory bank files become large or need reorganization,
  follow this preservation protocol:

  #### 1. Archive Creation Process
  - **Create `archive.md`** - A comprehensive archive file
  containing ALL historical content
  - **Preserve Everything** - Move content to archive, never
  delete
  - **Maintain Context** - Keep complete project history and
  decision trails
  - **Reference Clearly** - Add clear references to archived
  content location

  #### 2. Content to Preserve in `archive.md`
  - Complete project history with dates
  - All major milestones and technical achievements
  - Critical fixes and their context
  - Deployment status updates and infrastructure information
  - Implementation details for major features
  - Security audits and performance optimization records
  - Testing completion records and results
  - All decision context and technical debt
  - Problem-solving approaches and lessons learned

  #### 3. Memory Bank File Structure After Archiving
  - **activeContext.md**: Current status with clear reference to
  historical archive
  - **archive.md**: Complete historical record preserved for
  future reference
  - **All other memory bank files**: Remain intact
  (decisionLog.md, systemPatterns.md, techContext.md,
  productContext.md, progress.md)

  #### 4. Archive Update Pattern
  ```markdown
  **HISTORICAL NOTE**: All previous progress and context has been
  preserved in `/memory-bank/archive.md`. This archive contains
  the complete history of the project from [START_DATE] through
  [END_DATE], including all major milestones, fixes, and
  implementations.

  **MEMORY BANK PRESERVATION: [DATE]**
  - ✅ **HISTORICAL ARCHIVE CREATED**: All previous memory bank
  content preserved in `archive.md`
  - ✅ **CONTENT PRESERVATION**: No historical information was
  deleted - only moved to archive
  - ✅ **CONTEXT CONTINUITY**: Complete project history maintained
   for future reference
  - ✅ **DECISION TRAIL**: All technical decisions and fixes
  preserved in archive

  5. Benefits of This System

  - Complete History: Never lose valuable project knowledge
  - Clean Current Context: Keep active files focused on current
  status
  - Easy Reference: Historical decisions and fixes remain
  accessible
  - Context Preservation: Maintain understanding of why decisions
  were made
  - Technical Debt Tracking: Keep record of known issues and their
   solutions

  6. When to Create Archives

  - Memory bank files exceed 500 lines
  - Major project phases complete
  - Significant architecture changes occur
  - Project handoffs or team changes
  - Annual project reviews

  Implementation Instructions for Claude

  When asked to "update memory bank" and files are large:

  1. NEVER delete any existing content
  2. Create archive.md with ALL historical content
  3. Update activeContext.md with current status + archive
  reference
  4. Preserve all other memory bank files intact
  5. Confirm preservation completion to user

  This ensures project knowledge is never lost while maintaining
  clean, usable memory bank files for ongoing development.
