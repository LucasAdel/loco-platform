# Loco Platform - Development Server Scripts

This directory contains convenient scripts to run your Loco Platform development environment with both frontend and backend servers.

## Quick Start

### Unix/Linux/macOS

1. **Setup the alias (one-time setup):**
   ```bash
   ./setup-alias.sh
   source ~/.zshrc  # or ~/.bashrc
   ```

2. **Run the application:**
   ```bash
   run              # Start both servers
   run stop         # Stop all servers
   run restart      # Restart all servers
   run status       # Show server status
   run help         # Show help
   ```

### Windows (PowerShell)

```powershell
.\run.ps1           # Start both servers
.\run.ps1 stop      # Stop all servers
.\run.ps1 restart   # Restart all servers
.\run.ps1 status    # Show server status
.\run.ps1 help      # Show help
```

## Available Scripts

| Script | Platform | Description |
|--------|----------|-------------|
| `run` | Unix/Linux/macOS | Main development server launcher |
| `run.ps1` | Windows | PowerShell version of the launcher |
| `setup-alias.sh` | Unix/Linux/macOS | Sets up shell alias for convenience |

## Commands

### `start` (default)
Starts both frontend and backend servers:
- **Backend**: http://localhost:3000 (Axum/Rust)
- **Frontend**: http://localhost:8080 (Dioxus/WASM)

### `stop`
Gracefully stops all running servers and cleans up processes.

### `restart`
Stops all servers and starts them again.

### `status`
Shows current status of both servers and their URLs.

### `logs`
Shows server logs (Unix/macOS only):
```bash
run logs backend   # Show backend logs
run logs frontend  # Show frontend logs
```

### `help`
Displays usage information and available commands.

## Features

### 🚀 Smart Startup
- Automatically detects and kills existing processes on required ports
- Starts backend first, then frontend with proper timing
- Monitors server startup and reports when ready
- Automatically opens browser when frontend is ready

### 📊 Process Management
- Tracks server process IDs for clean shutdown
- Handles Ctrl+C gracefully to stop all servers
- Monitors server health and reports crashes
- Port conflict detection and resolution

### 📝 Logging
- Separate log files for frontend and backend
- Real-time log viewing with `run logs` command
- Colored output for better readability
- Detailed startup and error information

### 🔧 Cross-Platform Support
- Native shell script for Unix/Linux/macOS
- PowerShell script for Windows
- Automatic shell detection and alias setup
- Consistent command interface across platforms

## Server Details

### Backend Server (Axum)
- **URL**: http://localhost:3000
- **Technology**: Rust + Axum framework
- **Features**: RESTful API, WebSocket support, database integration
- **Log file**: `backend.log`

### Frontend Server (Dioxus)
- **URL**: http://localhost:8080
- **Technology**: Rust + Dioxus (compiles to WebAssembly)
- **Features**: Hot reload, modern UI, real-time updates
- **Log file**: `frontend.log`

## Troubleshooting

### Port Already in Use
The script automatically detects and kills processes using required ports (3000, 8080).

### Build Errors
Check the respective log files:
```bash
run logs backend   # For backend issues
run logs frontend  # For frontend issues
```

### Slow Frontend Startup
The initial frontend build can take 2-3 minutes due to Rust compilation and WASM generation. Subsequent builds are much faster with hot reload.

### Permission Issues (macOS/Linux)
Make sure the scripts are executable:
```bash
chmod +x run setup-alias.sh
```

## Environment Variables

The scripts support these environment variables:

- `RUST_LOG`: Set logging level (default: debug)
- `FRONTEND_PORT`: Frontend port (default: 8080)
- `BACKEND_PORT`: Backend port (default: 3000)

Example:
```bash
RUST_LOG=info FRONTEND_PORT=3001 run
```

## Development Workflow

1. **Start development**:
   ```bash
   run
   ```

2. **Make changes**: Edit your Rust code - the frontend has hot reload enabled

3. **Check logs**: Monitor both servers for errors
   ```bash
   run logs backend
   run logs frontend
   ```

4. **Stop when done**:
   ```bash
   run stop
   ```

## Integration with IDEs

### VS Code
Add this to your VS Code tasks.json:
```json
{
    "label": "Start Loco Platform",
    "type": "shell",
    "command": "./run",
    "group": "build",
    "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "new"
    }
}
```

### IntelliJ/CLion
Create a new Shell Script run configuration pointing to the `run` script.

## Contributing

When adding new features to the scripts:

1. Maintain cross-platform compatibility
2. Add appropriate error handling
3. Update this documentation
4. Test on multiple operating systems
5. Follow existing code style and patterns

## Support

For issues with the run scripts:
1. Check the log files first
2. Ensure all dependencies are installed (Rust, Dioxus CLI)
3. Verify ports 3000 and 8080 are available
4. Try running individual components manually for debugging