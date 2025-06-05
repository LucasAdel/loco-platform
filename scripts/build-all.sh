#!/bin/bash
# Build script for all targets

set -e

echo "🚀 Building Loco Platform for all targets..."

# Make build scripts executable
chmod +x scripts/build-web.sh
chmod +x scripts/build-desktop.sh

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf dist/
cargo clean

# Build for web
echo ""
echo "=================="
echo "Building for Web..."
echo "=================="
./scripts/build-web.sh

# Build for desktop
echo ""
echo "======================"
echo "Building for Desktop..."
echo "======================"
./scripts/build-desktop.sh

echo ""
echo "✅ All builds complete!"
echo ""
echo "Outputs:"
echo "  - Web:     dist/"
echo "  - Desktop: dist/desktop/"