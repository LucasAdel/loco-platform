#!/bin/bash
# Quick database connection test

echo "🔍 Testing database connection..."

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Parse DATABASE_URL
if [ ! -z "$DATABASE_URL" ]; then
    DB_USER=$(echo $DATABASE_URL | sed -n 's|.*://\([^:]*\):.*|\1|p')
    DB_PASSWORD=$(echo $DATABASE_URL | sed -n 's|.*://[^:]*:\([^@]*\)@.*|\1|p')
    DB_HOST=$(echo $DATABASE_URL | sed -n 's|.*@\([^:]*\):.*|\1|p')
    DB_PORT=$(echo $DATABASE_URL | sed -n 's|.*:\([0-9]*\)/.*|\1|p')
    DB_NAME=$(echo $DATABASE_URL | sed -n 's|.*/\([^?]*\).*|\1|p')
fi

echo "📊 Database configuration:"
echo "   Host: ${DB_HOST}:${DB_PORT}"
echo "   Database: ${DB_NAME}"
echo "   User: ${DB_USER}"

# Test connection
if PGPASSWORD=${DB_PASSWORD} psql -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} -d ${DB_NAME} -c "SELECT version();" > /dev/null 2>&1; then
    echo "✅ Database connection successful!"
    
    # List tables
    echo -e "\n📋 Current tables:"
    PGPASSWORD=${DB_PASSWORD} psql -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} -d ${DB_NAME} -c "\dt"
else
    echo "❌ Database connection failed!"
    exit 1
fi