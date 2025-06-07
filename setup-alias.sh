#!/bin/bash

# Setup script to create 'run' alias for Loco Platform

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUN_SCRIPT="$SCRIPT_DIR/run"

# Detect shell and profile file
if [[ "$SHELL" == *"zsh"* ]]; then
    PROFILE_FILE="$HOME/.zshrc"
    SHELL_NAME="zsh"
elif [[ "$SHELL" == *"bash"* ]]; then
    if [[ -f "$HOME/.bash_profile" ]]; then
        PROFILE_FILE="$HOME/.bash_profile"
    else
        PROFILE_FILE="$HOME/.bashrc"
    fi
    SHELL_NAME="bash"
else
    echo "Unsupported shell: $SHELL"
    echo "Please manually add this alias to your shell profile:"
    echo "alias run='$RUN_SCRIPT'"
    exit 1
fi

echo "Setting up 'run' alias for Loco Platform..."
echo "Shell detected: $SHELL_NAME"
echo "Profile file: $PROFILE_FILE"

# Check if alias already exists
if grep -q "alias run=" "$PROFILE_FILE" 2>/dev/null; then
    echo "Alias 'run' already exists in $PROFILE_FILE"
    echo "Current alias:"
    grep "alias run=" "$PROFILE_FILE"
    echo
    read -p "Do you want to update it? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Remove existing alias
        sed -i.bak "/alias run=/d" "$PROFILE_FILE"
        echo "Removed existing alias"
    else
        echo "Alias setup cancelled"
        exit 0
    fi
fi

# Add new alias
echo "" >> "$PROFILE_FILE"
echo "# Loco Platform development server alias" >> "$PROFILE_FILE"
echo "alias run='$RUN_SCRIPT'" >> "$PROFILE_FILE"
echo "" >> "$PROFILE_FILE"

echo "✅ Alias added to $PROFILE_FILE"
echo
echo "To use the alias immediately, run:"
echo "  source $PROFILE_FILE"
echo
echo "Or open a new terminal window."
echo
echo "Usage examples:"
echo "  run              # Start both servers"
echo "  run stop         # Stop all servers"
echo "  run restart      # Restart all servers"
echo "  run status       # Show server status"
echo "  run logs backend # Show backend logs"
echo "  run help         # Show help"
echo
echo "Servers will be available at:"
echo "  Frontend: http://localhost:8080"
echo "  Backend:  http://localhost:3070"