#!/bin/bash
# Build script for native desktop target

set -e

echo "🖥️  Building Loco Platform for Desktop..."

# Detect current platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    if [[ $(uname -m) == 'arm64' ]]; then
        TARGET="aarch64-apple-darwin"
    else
        TARGET="x86_64-apple-darwin"
    fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    TARGET="x86_64-unknown-linux-gnu"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    TARGET="x86_64-pc-windows-msvc"
else
    echo "Unsupported platform: $OSTYPE"
    exit 1
fi

echo "Detected target: $TARGET"

# Ensure target is installed
rustup target add $TARGET

# Build the backend with SSR support
echo "Building backend with SSR..."
cd backend
cargo build --release

# Build the app with SSR features
echo "Building app with SSR..."
cd ../app
cargo build --release --features ssr

# Create distribution directory
mkdir -p ../dist/desktop

# Copy binary
echo "Copying binary..."
cp ../target/release/backend ../dist/desktop/loco-platform

# Copy static assets
echo "Copying assets..."
mkdir -p ../dist/desktop/assets
cp -r assets/* ../dist/desktop/assets/ 2>/dev/null || true

# Create run script
cat > ../dist/desktop/run.sh << 'EOF'
#!/bin/bash
# Set environment variables
export RUST_LOG=info
export DATABASE_URL=${DATABASE_URL:-"postgresql://localhost/loco_platform"}
export BIND_ADDRESS=${BIND_ADDRESS:-"127.0.0.1:3070"}

# Run the application
./loco-platform
EOF

chmod +x ../dist/desktop/run.sh
chmod +x ../dist/desktop/loco-platform

echo "✅ Desktop build complete! Output in dist/desktop/"
echo "   Run './dist/desktop/run.sh' to start the application"