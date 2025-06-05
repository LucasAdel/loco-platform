# Loco Platform Setup Guide

## Prerequisites

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **PostgreSQL** (version 14 or higher)
   - macOS: `brew install postgresql`
   - Ubuntu: `sudo apt install postgresql postgresql-contrib`
   - Windows: Download from https://www.postgresql.org/download/windows/

3. **Node.js** (for frontend tooling)
   - Download from https://nodejs.org/

## Quick Start

### 1. Clone and Navigate
```bash
git clone <repository-url>
cd loco-platform
```

### 2. Install Rust Tools
```bash
# Install Dioxus CLI
cargo install dioxus-cli

# Install SeaORM CLI (optional, for database management)
cargo install sea-orm-cli

# Install cargo-watch (optional, for auto-reload)
cargo install cargo-watch
```

### 3. Environment Setup
```bash
# Copy environment template
cp .env.example .env

# Edit .env with your configuration
# Required: SUPABASE_URL, SUPABASE_ANON_KEY, JWT_SECRET
```

### 4. Database Setup
```bash
# Make scripts executable
chmod +x scripts/*.sh

# Set up database (creates database and runs migrations)
./scripts/setup-database.sh
```

### 5. Git Hooks Setup
```bash
# Install pre-commit hooks
./scripts/setup-hooks.sh
```

### 6. Verify Setup
```bash
# Run verification script
./scripts/test-setup.sh
```

## Development

### Start Backend
```bash
# Terminal 1
cargo run -p backend

# Or with auto-reload
cargo watch -x "run -p backend"
```

### Start Frontend
```bash
# Terminal 2
cd frontend-dioxus
dx serve --hot-reload

# Or from root
dx serve --hot-reload --project frontend-dioxus
```

### Run Tests
```bash
# All tests
cargo test --all

# Specific package
cargo test -p backend
cargo test -p shared

# With output
cargo test --all -- --nocapture
```

### Code Quality
```bash
# Format code
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features

# Security audit
cargo audit
```

## Project Structure

```
loco-platform/
├── frontend-dioxus/    # Dioxus web application
├── backend/           # Axum REST API server
├── shared/            # Shared types and utilities
├── migrations/        # Database migrations
├── memory-bank/       # Project documentation
├── scripts/           # Utility scripts
└── tests/            # Integration tests
```

## Common Issues

### PostgreSQL Connection Failed
```bash
# Check if PostgreSQL is running
pg_isready

# Start PostgreSQL (macOS)
brew services start postgresql

# Start PostgreSQL (Linux)
sudo systemctl start postgresql
```

### Port Already in Use
```bash
# Backend defaults to port 3000
# Frontend defaults to port 8080

# Change in .env or use different port:
PORT=3001 cargo run -p backend
```

### WASM Target Missing
```bash
# Add WASM target for Dioxus
rustup target add wasm32-unknown-unknown
```

### Node Modules Missing
```bash
cd frontend-dioxus
npm install
```

## Features

- 🔐 **Multi-tenant Architecture** with Row-Level Security
- 🔑 **JWT Authentication** with Argon2 password hashing
- 🌐 **Supabase Integration** for auth and real-time features
- 🗺️ **Mapbox Integration** for location features
- 📱 **Responsive Design** with TailwindCSS
- 🚀 **Type-safe** full-stack with shared types
- 🧪 **Comprehensive Testing** setup
- 🪝 **Git Hooks** for code quality

## Next Steps

1. Configure Supabase project
2. Set up Mapbox account for mapping features
3. Review `checklist.md` for feature implementation status
4. Check `memory-bank/` for architecture decisions

## Support

- Check `CLAUDE.md` for project conventions
- Review `memory-bank/systemPatterns.md` for coding patterns
- See `checklist.md` for feature roadmap