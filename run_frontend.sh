#!/bin/bash

# Simple script to run the frontend without dx CLI
echo "🚀 Starting Loco Platform Frontend Development Server"
echo "Note: This is a simplified setup while dx CLI installs"

# Check if dx is available
if command -v dx &> /dev/null; then
    echo "Using Dioxus CLI..."
    dx serve
else
    echo "Dioxus CLI not available, using cargo with native web server..."
    echo "Backend should be running on http://localhost:3000"
    echo "Starting development setup..."
    
    # Try to build frontend first to check for errors
    cd frontend
    echo "Building frontend..."
    cargo check
    
    if [ $? -eq 0 ]; then
        echo "✅ Frontend builds successfully!"
        echo "📝 To run with hot reload, install Dioxus CLI:"
        echo "   cargo install dioxus-cli"
        echo "   dx serve"
        echo ""
        echo "🌐 Backend API is available at:"
        echo "   http://localhost:3000/api/jobs"
        echo "   http://localhost:3000/health"
        echo ""
        echo "You can test the backend with:"
        echo "   curl http://localhost:3000/api/jobs"
    else
        echo "❌ Frontend has compilation issues"
    fi
fi