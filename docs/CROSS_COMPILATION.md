# Cross-Compilation Guide

This guide explains how to build the Loco Platform for different targets.

## Supported Targets

### Web (WASM)
- **Target**: `wasm32-unknown-unknown`
- **Features**: Client-side rendering with Leptos
- **Output**: WebAssembly module + JavaScript bindings

### Desktop
- **macOS**: `aarch64-apple-darwin` (M1/M2), `x86_64-apple-darwin` (Intel)
- **Linux**: `x86_64-unknown-linux-gnu`
- **Windows**: `x86_64-pc-windows-msvc`
- **Features**: Server-side rendering with full backend

## Prerequisites

### Common Requirements
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cargo-leptos for development
cargo install cargo-leptos
```

### Web Build Requirements
```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install WASM tools
cargo install wasm-bindgen-cli
cargo install wasm-opt
```

### Desktop Build Requirements
```bash
# Add your platform's target
rustup target add aarch64-apple-darwin  # For Apple Silicon
rustup target add x86_64-apple-darwin    # For Intel Mac
rustup target add x86_64-unknown-linux-gnu  # For Linux
rustup target add x86_64-pc-windows-msvc    # For Windows
```

## Building

### Quick Build Commands

```bash
# Build for web
./scripts/build-web.sh

# Build for desktop
./scripts/build-desktop.sh

# Build all targets
./scripts/build-all.sh
```

### Manual Build Commands

#### Web Build
```bash
cd app
cargo build --target wasm32-unknown-unknown --release --features web

# Generate JavaScript bindings
wasm-bindgen \
  --out-dir ../dist/pkg \
  --target web \
  --no-typescript \
  ../target/wasm32-unknown-unknown/release/loco_app.wasm

# Optimise WASM
wasm-opt -Oz \
  -o ../dist/pkg/loco_app_bg.wasm \
  ../dist/pkg/loco_app_bg.wasm
```

#### Desktop Build
```bash
# Build backend
cd backend
cargo build --release

# Build app with SSR
cd ../app
cargo build --release --features ssr

# Run the application
cd ..
./target/release/backend
```

## Development

### Local Development Server
```bash
# For web development with hot reload
cargo leptos watch

# For desktop development
cargo run -p backend
```

### Cross-Platform Testing
```bash
# Test WASM build
cargo test --target wasm32-unknown-unknown

# Test desktop build
cargo test --all
```

## Configuration

### Cargo Configuration
The `.cargo/config.toml` file contains:
- Target-specific settings
- Linker configurations
- Build optimisations
- Convenient aliases

### Feature Flags

#### App Features
- `csr`: Client-side rendering (default)
- `ssr`: Server-side rendering
- `hydrate`: Hydration support
- `web`: Web-specific dependencies
- `desktop`: Desktop-specific dependencies

#### Backend Features
- Default features include all server capabilities

## Deployment

### Web Deployment
1. Build for web: `./scripts/build-web.sh`
2. Upload `dist/` directory to your web server
3. Configure server to serve `index.html` for all routes

### Desktop Deployment

#### macOS
1. Build: `./scripts/build-desktop.sh`
2. Create `.app` bundle (optional)
3. Sign with Apple Developer certificate
4. Distribute via DMG or App Store

#### Windows
1. Build: `./scripts/build-desktop.sh`
2. Create installer with NSIS or MSI
3. Sign with code signing certificate
4. Distribute via installer

#### Linux
1. Build: `./scripts/build-desktop.sh`
2. Create packages:
   - DEB for Debian/Ubuntu
   - RPM for Fedora/RHEL
   - AppImage for universal distribution
   - Snap or Flatpak

## Optimisation Tips

### WASM Optimisation
- Use `wasm-opt` with `-Oz` flag for size
- Enable LTO in release profile
- Strip debug symbols
- Use `panic = "abort"`

### Desktop Optimisation
- Use thin LTO for faster builds
- Profile-guided optimisation
- Static linking where appropriate
- Native CPU features

## Troubleshooting

### Common Issues

1. **WASM build fails**
   - Ensure all dependencies support WASM
   - Check for filesystem operations
   - Verify no native dependencies

2. **Desktop build fails**
   - Check system dependencies
   - Verify OpenSSL installation
   - Ensure correct target triple

3. **Cross-compilation fails**
   - Install cross-compilation toolchain
   - Configure linker correctly
   - Check target-specific dependencies

### Platform-Specific Notes

#### macOS
- Requires Xcode or Command Line Tools
- May need to accept Xcode license

#### Windows
- Requires Visual Studio or Build Tools
- Set up MSVC environment variables

#### Linux
- May need additional system libraries
- Install `pkg-config` and development headers

## CI/CD Integration

See `.github/workflows/cross-compile.yml` for automated builds across all platforms.