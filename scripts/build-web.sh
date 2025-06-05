#!/bin/bash
# Build script for WASM/Web target

set -e

echo "🌐 Building Loco Platform for Web (WASM)..."

# Ensure wasm target is installed
rustup target add wasm32-unknown-unknown

# Install required tools if not present
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

if ! command -v wasm-opt &> /dev/null; then
    echo "Installing wasm-opt..."
    cargo install wasm-opt
fi

# Build the app for WASM
echo "Building WASM target..."
cd app
cargo build --target wasm32-unknown-unknown --release --features web

# Generate JavaScript bindings
echo "Generating JavaScript bindings..."
wasm-bindgen \
    --out-dir ../dist/pkg \
    --target web \
    --no-typescript \
    ../target/wasm32-unknown-unknown/release/loco_app.wasm

# Optimise WASM file
echo "Optimising WASM file..."
wasm-opt -Oz \
    -o ../dist/pkg/loco_app_bg.wasm \
    ../dist/pkg/loco_app_bg.wasm

# Copy assets
echo "Copying assets..."
mkdir -p ../dist/assets
cp -r assets/* ../dist/assets/ 2>/dev/null || true

# Generate index.html
echo "Generating index.html..."
cat > ../dist/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Loco Platform - Australian Pharmacy Jobs</title>
    <link rel="stylesheet" href="/assets/output.css">
    <link rel="stylesheet" href="/assets/global.css">
</head>
<body>
    <div id="app"></div>
    <script type="module">
        import init from './pkg/loco_app.js';
        
        async function run() {
            await init();
        }
        
        run();
    </script>
</body>
</html>
EOF

echo "✅ Web build complete! Output in dist/"
echo "   Run 'python3 -m http.server 8080 -d dist' to test locally"