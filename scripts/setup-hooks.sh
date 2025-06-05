#!/bin/bash
# Setup script for git hooks

set -e

echo "🔧 Setting up git hooks..."

# Check if we're in a git repository
if [ ! -d .git ]; then
    echo "❌ Not in a git repository. Please run 'git init' first."
    exit 1
fi

# Configure git to use our hooks directory
git config core.hooksPath .githooks
echo "✅ Configured git to use .githooks directory"

# Install pre-commit if available
if command -v pre-commit >/dev/null 2>&1; then
    echo "📦 Installing pre-commit hooks..."
    pre-commit install
    pre-commit install --hook-type commit-msg
    pre-commit install --hook-type pre-push
    echo "✅ Pre-commit hooks installed"
else
    echo "ℹ️  pre-commit not installed"
    echo "   For enhanced hook management, install with:"
    echo "   pip install pre-commit"
fi

# Install required tools
echo ""
echo "🛠️  Checking required tools..."

# Check cargo-audit
if ! command -v cargo-audit >/dev/null 2>&1; then
    echo "📦 Installing cargo-audit..."
    cargo install cargo-audit
fi

# Check if Python is available for TOML validation
if ! command -v python3 >/dev/null 2>&1; then
    echo "⚠️  Python 3 not found. TOML validation will be skipped."
    echo "   Install Python 3 for full hook functionality."
fi

echo ""
echo "✅ Git hooks setup complete!"
echo ""
echo "Hooks installed:"
echo "  • pre-commit  - Runs formatting, linting, and checks"
echo "  • pre-push    - Runs tests and security audit"
echo "  • commit-msg  - Validates commit message format"
echo ""
echo "To bypass hooks (use sparingly):"
echo "  git commit --no-verify"
echo "  git push --no-verify"