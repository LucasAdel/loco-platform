#!/bin/bash

# Loco Platform - Leptos Development Server Launcher
# This script starts both the Leptos frontend and Axum backend servers

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[LOCO PLATFORM]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if a port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 0  # Port is in use
    else
        return 1  # Port is free
    fi
}

# Function to kill processes on specified ports
cleanup_ports() {
    print_status "Cleaning up existing processes..."
    
    # Kill processes on frontend port (3080)
    if check_port 3080; then
        print_warning "Killing existing process on port 3080"
        lsof -ti:3080 | xargs kill -9 2>/dev/null || true
    fi
    
    # Kill processes on backend port (3070)
    if check_port 3070; then
        print_warning "Killing existing process on port 3070"
        lsof -ti:3070 | xargs kill -9 2>/dev/null || true
    fi
    
    sleep 2
}

# Function to start backend server
start_backend() {
    print_status "Starting backend server (Axum)..."
    cd backend
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi
    
    # Start backend in background
    RUST_LOG=debug cargo run --bin backend > ../backend.log 2>&1 &
    BACKEND_PID=$!
    echo $BACKEND_PID > ../backend.pid
    
    cd ..
    print_success "Backend server starting on http://localhost:3070 (PID: $BACKEND_PID)"
}

# Function to start Leptos frontend
start_leptos() {
    print_status "Starting Leptos frontend server..."
    
    # Check if cargo-leptos is available
    if ! command -v cargo-leptos &> /dev/null; then
        print_error "cargo-leptos not found. Installing..."
        cargo install cargo-leptos
    fi
    
    # Start Leptos app
    print_status "Building and serving Leptos app..."
    cd app
    cargo leptos watch > ../frontend.log 2>&1 &
    FRONTEND_PID=$!
    echo $FRONTEND_PID > ../frontend.pid
    cd ..
    
    print_success "Leptos server starting on http://localhost:3080 (PID: $FRONTEND_PID)"
}

# Function to wait for servers to be ready
wait_for_servers() {
    print_status "Waiting for servers to start..."
    
    # Wait for backend (max 30 seconds)
    backend_ready=false
    for i in {1..30}; do
        if check_port 3070; then
            backend_ready=true
            break
        fi
        sleep 1
    done
    
    # Wait for frontend (max 60 seconds - compilation takes time)
    frontend_ready=false
    for i in {1..60}; do
        if check_port 3080; then
            frontend_ready=true
            break
        fi
        sleep 1
    done
    
    # Report status
    if $backend_ready; then
        print_success "Backend server is ready on http://localhost:3070"
    else
        print_warning "Backend server may still be starting. Check backend.log for details."
    fi
    
    if $frontend_ready; then
        print_success "Leptos server is ready on http://localhost:3080"
        print_status "Opening browser..."
        # Open browser based on OS
        case "$(uname -s)" in
            Darwin)  open "http://localhost:3080" ;;
            Linux)   xdg-open "http://localhost:3080" ;;
            CYGWIN*|MINGW*) start "http://localhost:3080" ;;
        esac
    else
        print_warning "Leptos server may still be compiling. Check frontend.log for details."
        print_status "You can manually open http://localhost:3080 when ready."
    fi
}

# Function to stop servers
stop_servers() {
    print_status "Stopping servers..."
    
    # Stop frontend
    if [ -f frontend.pid ]; then
        FRONTEND_PID=$(cat frontend.pid)
        if kill -0 $FRONTEND_PID 2>/dev/null; then
            kill $FRONTEND_PID
            print_success "Leptos server stopped (PID: $FRONTEND_PID)"
        fi
        rm -f frontend.pid
    fi
    
    # Stop backend
    if [ -f backend.pid ]; then
        BACKEND_PID=$(cat backend.pid)
        if kill -0 $BACKEND_PID 2>/dev/null; then
            kill $BACKEND_PID
            print_success "Backend server stopped (PID: $BACKEND_PID)"
        fi
        rm -f backend.pid
    fi
    
    # Force cleanup ports
    cleanup_ports
    
    print_success "All servers stopped"
}

# Main function to start all servers
start_servers() {
    print_status "🚀 Starting Loco Platform Development Environment (Leptos + Axum)"
    echo
    
    # Change to project directory
    cd "$(dirname "$0")"
    
    # Cleanup any existing processes
    cleanup_ports
    
    # Start servers
    start_backend
    sleep 3  # Give backend a head start
    start_leptos
    
    # Wait for servers and show status
    wait_for_servers
    
    print_status "Development environment is ready!"
    print_status "  Frontend (Leptos): http://localhost:3080"
    print_status "  Backend (Axum):    http://localhost:3070"
    print_status "Press Ctrl+C to stop all servers"
    
    # Keep script running and handle Ctrl+C
    trap 'echo; print_status "Shutting down..."; stop_servers; exit 0' INT
    
    # Monitor servers
    while true; do
        sleep 5
        
        # Check if processes are still running
        if [ -f backend.pid ]; then
            BACKEND_PID=$(cat backend.pid)
            if ! kill -0 $BACKEND_PID 2>/dev/null; then
                print_error "Backend server crashed! Check backend.log"
                break
            fi
        fi
        
        if [ -f frontend.pid ]; then
            FRONTEND_PID=$(cat frontend.pid)
            if ! kill -0 $FRONTEND_PID 2>/dev/null; then
                print_error "Leptos server crashed! Check frontend.log"
                break
            fi
        fi
    done
}

# Main script logic
case "${1:-start}" in
    start|"")
        start_servers
        ;;
    stop)
        stop_servers
        ;;
    *)
        print_error "Unknown command: $1"
        echo "Usage: $0 [start|stop]"
        exit 1
        ;;
esac