#!/bin/bash
# Database setup script for Loco Platform

set -e

echo "🚀 Setting up Loco Platform database..."

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Parse DATABASE_URL if available
if [ ! -z "$DATABASE_URL" ]; then
    # Extract components from DATABASE_URL
    # Format: postgresql://user:password@host:port/database
    DB_USER=$(echo $DATABASE_URL | sed -n 's|.*://\([^:]*\):.*|\1|p')
    DB_PASSWORD=$(echo $DATABASE_URL | sed -n 's|.*://[^:]*:\([^@]*\)@.*|\1|p')
    DB_HOST=$(echo $DATABASE_URL | sed -n 's|.*@\([^:]*\):.*|\1|p')
    DB_PORT=$(echo $DATABASE_URL | sed -n 's|.*:\([0-9]*\)/.*|\1|p')
    DB_NAME=$(echo $DATABASE_URL | sed -n 's|.*/\([^?]*\).*|\1|p')
else
    # Default values if not set
    DB_HOST=${DB_HOST:-localhost}
    DB_PORT=${DB_PORT:-5432}
    DB_NAME=${DB_NAME:-loco_platform}
    DB_USER=${DB_USER:-postgres}
    DB_PASSWORD=${DB_PASSWORD:-password}
    
    # Construct database URL
    export DATABASE_URL="postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
fi

echo "📊 Database configuration:"
echo "   Host: ${DB_HOST}:${DB_PORT}"
echo "   Database: ${DB_NAME}"
echo "   User: ${DB_USER}"

# Check if PostgreSQL is running
if ! pg_isready -h ${DB_HOST} -p ${DB_PORT} > /dev/null 2>&1; then
    echo "❌ PostgreSQL is not running or not accessible"
    echo "   Please ensure PostgreSQL is installed and running"
    exit 1
fi

# Create database if it doesn't exist
echo "🔧 Creating database if not exists..."
PGPASSWORD=${DB_PASSWORD} psql -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} -tc "SELECT 1 FROM pg_database WHERE datname = '${DB_NAME}'" | grep -q 1 || \
PGPASSWORD=${DB_PASSWORD} psql -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} -c "CREATE DATABASE ${DB_NAME}"

# Enable UUID extension
echo "🔧 Enabling UUID extension..."
PGPASSWORD=${DB_PASSWORD} psql -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} -d ${DB_NAME} -c "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";"

# Run migrations
echo "🔄 Running database migrations..."
cd migrations
cargo run -- up
cd ..

echo "✅ Database setup complete!"
echo ""
echo "To seed the database with sample data, run:"
echo "   cargo run -p backend"
echo ""
echo "The backend will automatically seed the database on first run in development mode."