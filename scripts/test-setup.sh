#!/bin/bash
# Test script to verify the Loco Platform setup

set -e

echo "🔍 Loco Platform Setup Verification Script"
echo "========================================="

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check Rust crate
check_crate() {
    if cargo tree -p "$1" >/dev/null 2>&1; then
        echo "✅ $1 crate found"
    else
        echo "❌ $1 crate not found"
        return 1
    fi
}

# Check prerequisites
echo -e "\n📋 Checking prerequisites..."

if command_exists rustc; then
    echo "✅ Rust installed: $(rustc --version)"
else
    echo "❌ Rust not installed"
    exit 1
fi

if command_exists cargo; then
    echo "✅ Cargo installed: $(cargo --version)"
else
    echo "❌ Cargo not installed"
    exit 1
fi

if command_exists sea-orm-cli; then
    echo "✅ SeaORM CLI installed"
else
    echo "⚠️  SeaORM CLI not installed (optional)"
fi

if command_exists dx; then
    echo "✅ Dioxus CLI installed"
else
    echo "⚠️  Dioxus CLI not installed (run: cargo install dioxus-cli)"
fi

# Check project structure
echo -e "\n📁 Checking project structure..."

DIRS=("frontend-dioxus" "backend" "shared" "migrations" "memory-bank" "scripts" "tests")
for dir in "${DIRS[@]}"; do
    if [ -d "$dir" ]; then
        echo "✅ $dir/ directory exists"
    else
        echo "❌ $dir/ directory missing"
    fi
done

# Check key files
echo -e "\n📄 Checking key files..."

FILES=("Cargo.toml" "CLAUDE.md" "checklist.md" ".env.example")
for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
    fi
done

# Check .env file
if [ -f ".env" ]; then
    echo "✅ .env file exists"
else
    echo "⚠️  .env file missing - copy from .env.example"
fi

# Check workspace configuration
echo -e "\n🔧 Checking workspace configuration..."

if cargo metadata --no-deps >/dev/null 2>&1; then
    echo "✅ Cargo workspace is valid"
    
    # List workspace members
    echo "   Workspace members:"
    cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[]' | sed 's/^/   - /'
else
    echo "❌ Cargo workspace configuration error"
fi

# Check dependencies
echo -e "\n📦 Checking key dependencies..."

cd backend
check_crate "axum"
check_crate "sea-orm"
check_crate "tokio"
check_crate "tower"
check_crate "jsonwebtoken"
check_crate "argon2"
cd ..

cd frontend-dioxus
check_crate "dioxus"
check_crate "dioxus-router"
check_crate "fermi"
cd ..

cd shared
check_crate "serde"
check_crate "uuid"
check_crate "chrono"
cd ..

# Check pre-commit hooks
echo -e "\n🪝 Checking git hooks..."

if [ -f ".git/hooks/pre-commit" ]; then
    echo "✅ Pre-commit hook installed"
else
    echo "⚠️  Pre-commit hook not installed (run: ./scripts/setup-hooks.sh)"
fi

# Check database connection (if PostgreSQL is running)
echo -e "\n🗄️  Checking database..."

if [ -f ".env" ]; then
    source .env
    if pg_isready -h ${DB_HOST:-localhost} -p ${DB_PORT:-5432} >/dev/null 2>&1; then
        echo "✅ PostgreSQL is running"
        
        # Check if database exists
        if PGPASSWORD=${DB_PASSWORD:-password} psql -h ${DB_HOST:-localhost} -p ${DB_PORT:-5432} -U ${DB_USER:-postgres} -lqt | cut -d \| -f 1 | grep -qw ${DB_NAME:-loco_platform}; then
            echo "✅ Database '${DB_NAME:-loco_platform}' exists"
        else
            echo "⚠️  Database '${DB_NAME:-loco_platform}' does not exist (run: ./scripts/setup-database.sh)"
        fi
    else
        echo "⚠️  PostgreSQL is not running"
    fi
else
    echo "⚠️  Cannot check database - .env file missing"
fi

# Check if the project builds
echo -e "\n🏗️  Checking if project builds..."

if cargo check --workspace 2>/dev/null; then
    echo "✅ Project builds successfully"
else
    echo "❌ Build errors found"
fi

# Summary
echo -e "\n📊 Setup Summary"
echo "==============="
echo "✅ = Ready"
echo "⚠️  = Optional/Warning"
echo "❌ = Action required"

echo -e "\n📝 Next steps:"
echo "1. Copy .env.example to .env and configure"
echo "2. Install optional tools: cargo install dioxus-cli sea-orm-cli"
echo "3. Run database setup: ./scripts/setup-database.sh"
echo "4. Install git hooks: ./scripts/setup-hooks.sh"
echo "5. Start development:"
echo "   - Backend: cargo run -p backend"
echo "   - Frontend: cd frontend-dioxus && dx serve"

echo -e "\n✨ Happy coding!"