#!/bin/bash

# Loco Platform Test Runner
# Rust-native testing with fantoccini + wasm-bindgen-test

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🦀 Loco Platform Test Runner${NC}"
echo "==============================="

# Configuration
BACKEND_PORT=${BACKEND_PORT:-3070}
FRONTEND_PORT=${FRONTEND_PORT:-3080}
WEBDRIVER_PORT=${WEBDRIVER_PORT:-4444}
HEADLESS=${HEADLESS:-true}
TEST_TYPE=${1:-"all"}

# Function to check if port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 0  # Port is in use
    else
        return 1  # Port is free
    fi
}

# Function to start WebDriver (Selenium)
start_webdriver() {
    echo -e "${YELLOW}Starting WebDriver (Selenium)...${NC}"
    
    if ! command -v java &> /dev/null; then
        echo -e "${RED}Error: Java is required for Selenium WebDriver${NC}"
        echo "Please install Java 8+ and try again"
        exit 1
    fi
    
    # Download Selenium if not present
    SELENIUM_JAR="selenium-server-4.15.0.jar"
    if [ ! -f "$SELENIUM_JAR" ]; then
        echo "Downloading Selenium WebDriver..."
        curl -L -o "$SELENIUM_JAR" "https://github.com/SeleniumHQ/selenium/releases/download/selenium-4.15.0/selenium-server-4.15.0.jar"
    fi
    
    # Download ChromeDriver if not present
    if ! command -v chromedriver &> /dev/null; then
        echo "ChromeDriver not found. Please install ChromeDriver:"
        echo "  brew install chromedriver  # macOS"
        echo "  apt install chromium-chromedriver  # Ubuntu"
        echo "  Or download from: https://chromedriver.chromium.org/"
        exit 1
    fi
    
    # Start Selenium in background
    java -jar "$SELENIUM_JAR" standalone --port $WEBDRIVER_PORT > webdriver.log 2>&1 &
    WEBDRIVER_PID=$!
    
    # Wait for WebDriver to start
    echo "Waiting for WebDriver to start..."
    for i in {1..30}; do
        if check_port $WEBDRIVER_PORT; then
            echo -e "${GREEN}✓ WebDriver started on port $WEBDRIVER_PORT${NC}"
            break
        fi
        sleep 1
    done
    
    if ! check_port $WEBDRIVER_PORT; then
        echo -e "${RED}Error: WebDriver failed to start${NC}"
        exit 1
    fi
}

# Function to start backend server
start_backend() {
    echo -e "${YELLOW}Starting backend server...${NC}"
    
    if check_port $BACKEND_PORT; then
        echo -e "${GREEN}✓ Backend already running on port $BACKEND_PORT${NC}"
        return
    fi
    
    cd ../backend
    cargo run > ../tests/backend.log 2>&1 &
    BACKEND_PID=$!
    cd ../tests
    
    # Wait for backend to start
    echo "Waiting for backend to start..."
    for i in {1..30}; do
        if curl -s "http://localhost:$BACKEND_PORT/health" > /dev/null 2>&1; then
            echo -e "${GREEN}✓ Backend started on port $BACKEND_PORT${NC}"
            break
        fi
        sleep 1
    done
    
    if ! curl -s "http://localhost:$BACKEND_PORT/health" > /dev/null 2>&1; then
        echo -e "${RED}Error: Backend failed to start${NC}"
        exit 1
    fi
}

# Function to start frontend server
start_frontend() {
    echo -e "${YELLOW}Starting frontend server...${NC}"
    
    if check_port $FRONTEND_PORT; then
        echo -e "${GREEN}✓ Frontend already running on port $FRONTEND_PORT${NC}"
        return
    fi
    
    cd ../
    # Use the static server for reliable testing
    python3 -m http.server $FRONTEND_PORT --directory static > tests/frontend.log 2>&1 &
    FRONTEND_PID=$!
    cd tests
    
    # Wait for frontend to start
    echo "Waiting for frontend to start..."
    for i in {1..15}; do
        if curl -s "http://localhost:$FRONTEND_PORT" > /dev/null 2>&1; then
            echo -e "${GREEN}✓ Frontend started on port $FRONTEND_PORT${NC}"
            break
        fi
        sleep 1
    done
    
    if ! curl -s "http://localhost:$FRONTEND_PORT" > /dev/null 2>&1; then
        echo -e "${RED}Error: Frontend failed to start${NC}"
        exit 1
    fi
}

# Function to run unit tests
run_unit_tests() {
    echo -e "${BLUE}Running Unit Tests...${NC}"
    cargo test --lib
}

# Function to run integration tests
run_integration_tests() {
    echo -e "${BLUE}Running Integration Tests...${NC}"
    TEST_BASE_URL="http://localhost:$FRONTEND_PORT" \
    TEST_API_URL="http://localhost:$BACKEND_PORT" \
    WEBDRIVER_URL="http://localhost:$WEBDRIVER_PORT" \
    TEST_HEADLESS="$HEADLESS" \
    cargo test --test integration_tests
}

# Function to run E2E tests
run_e2e_tests() {
    echo -e "${BLUE}Running E2E Tests...${NC}"
    TEST_BASE_URL="http://localhost:$FRONTEND_PORT" \
    TEST_API_URL="http://localhost:$BACKEND_PORT" \
    WEBDRIVER_URL="http://localhost:$WEBDRIVER_PORT" \
    TEST_HEADLESS="$HEADLESS" \
    cargo test --test e2e_tests
}

# Function to run WASM tests
run_wasm_tests() {
    echo -e "${BLUE}Running WASM Tests...${NC}"
    
    if ! command -v wasm-pack &> /dev/null; then
        echo "Installing wasm-pack..."
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    fi
    
    wasm-pack test --headless --chrome --test wasm_tests
}

# Function to cleanup processes
cleanup() {
    echo -e "${YELLOW}Cleaning up...${NC}"
    
    if [ ! -z "$WEBDRIVER_PID" ]; then
        kill $WEBDRIVER_PID 2>/dev/null || true
        echo "✓ WebDriver stopped"
    fi
    
    if [ ! -z "$BACKEND_PID" ]; then
        kill $BACKEND_PID 2>/dev/null || true
        echo "✓ Backend stopped" 
    fi
    
    if [ ! -z "$FRONTEND_PID" ]; then
        kill $FRONTEND_PID 2>/dev/null || true
        echo "✓ Frontend stopped"
    fi
    
    # Kill any remaining processes on our ports
    lsof -ti:$WEBDRIVER_PORT | xargs kill -9 2>/dev/null || true
    lsof -ti:$BACKEND_PORT | xargs kill -9 2>/dev/null || true
    lsof -ti:$FRONTEND_PORT | xargs kill -9 2>/dev/null || true
}

# Set trap to cleanup on exit
trap cleanup EXIT

# Create screenshots directory
mkdir -p target/screenshots

# Main test execution
case "$TEST_TYPE" in
    "unit")
        run_unit_tests
        ;;
    "integration")
        start_backend
        run_integration_tests
        ;;
    "e2e")
        start_webdriver
        start_backend
        start_frontend
        run_e2e_tests
        ;;
    "wasm")
        run_wasm_tests
        ;;
    "all")
        echo -e "${BLUE}Running All Tests...${NC}"
        
        # Unit tests (no servers needed)
        run_unit_tests
        
        # Integration tests (backend only)
        start_backend
        run_integration_tests
        
        # E2E tests (all servers)
        start_webdriver
        start_frontend
        run_e2e_tests
        
        # WASM tests
        run_wasm_tests
        ;;
    *)
        echo "Usage: $0 [unit|integration|e2e|wasm|all]"
        echo ""
        echo "Test Types:"
        echo "  unit        - Run unit tests only"
        echo "  integration - Run API integration tests"
        echo "  e2e         - Run end-to-end browser tests"
        echo "  wasm        - Run WebAssembly tests"
        echo "  all         - Run all test suites (default)"
        echo ""
        echo "Environment Variables:"
        echo "  HEADLESS=false     - Run browser tests in headed mode"
        echo "  BACKEND_PORT=3070  - Backend server port"
        echo "  FRONTEND_PORT=3080 - Frontend server port"
        exit 1
        ;;
esac

echo -e "${GREEN}🎉 All tests completed successfully!${NC}"