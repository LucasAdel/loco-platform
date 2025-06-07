# Loco Platform - Development Server Launcher (PowerShell)
# This script starts both frontend (Dioxus) and backend (Axum) servers

param(
    [Parameter(Position=0)]
    [string]$Command = "start"
)

# Colors
$Red = "Red"
$Green = "Green"
$Blue = "Blue"
$Yellow = "Yellow"

function Write-Status {
    param([string]$Message)
    Write-Host "[LOCO PLATFORM] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Test-Port {
    param([int]$Port)
    $connection = Test-NetConnection -ComputerName localhost -Port $Port -InformationLevel Quiet -WarningAction SilentlyContinue
    return $connection
}

function Stop-Servers {
    Write-Status "Stopping servers..."
    
    # Stop processes on ports
    $frontendProcess = Get-Process | Where-Object {$_.ProcessName -eq "dx" -or $_.ProcessName -eq "dioxus"}
    if ($frontendProcess) {
        $frontendProcess | Stop-Process -Force
        Write-Success "Frontend server stopped"
    }
    
    $backendProcess = Get-Process | Where-Object {$_.ProcessName -eq "backend" -or $_.ProcessName -eq "cargo"}
    if ($backendProcess) {
        $backendProcess | Stop-Process -Force
        Write-Success "Backend server stopped"
    }
    
    Write-Success "All servers stopped"
}

function Start-Backend {
    Write-Status "Starting backend server (Axum)..."
    
    if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error "Cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    }
    
    Set-Location backend
    $env:RUST_LOG = "debug"
    Start-Process -FilePath "cargo" -ArgumentList "run", "--bin", "backend" -WindowStyle Hidden
    Set-Location ..
    
    Write-Success "Backend server starting on http://localhost:3070"
}

function Start-Frontend {
    Write-Status "Starting frontend server (Dioxus)..."
    
    if (!(Get-Command dx -ErrorAction SilentlyContinue)) {
        Write-Error "Dioxus CLI not found. Installing..."
        cargo install dioxus-cli
    }
    
    Start-Process -FilePath "dx" -ArgumentList "serve", "--package", "frontend", "--platform", "web", "--hot-reload", "true", "--port", "3080" -WindowStyle Hidden
    
    Write-Success "Frontend server starting on http://localhost:3080"
}

function Wait-ForServers {
    Write-Status "Waiting for servers to start..."
    
    # Wait for backend
    $backendReady = $false
    for ($i = 1; $i -le 30; $i++) {
        if (Test-Port -Port 3070) {
            $backendReady = $true
            break
        }
        Start-Sleep -Seconds 1
    }
    
    # Wait for frontend
    $frontendReady = $false
    for ($i = 1; $i -le 60; $i++) {
        if (Test-Port -Port 3080) {
            $frontendReady = $true
            break
        }
        Start-Sleep -Seconds 1
    }
    
    if ($backendReady) {
        Write-Success "Backend server is ready on http://localhost:3070"
    } else {
        Write-Warning "Backend server may still be starting."
    }
    
    if ($frontendReady) {
        Write-Success "Frontend server is ready on http://localhost:3080"
        Write-Status "Opening browser..."
        Start-Process "http://localhost:3080"
    } else {
        Write-Warning "Frontend server may still be compiling."
    }
}

function Show-Status {
    Write-Host ""
    Write-Status "Server Status:"
    
    if (Test-Port -Port 3070) {
        Write-Host "  Backend:  " -NoNewline
        Write-Host "✓ Running" -ForegroundColor Green -NoNewline
        Write-Host " on http://localhost:3070"
    } else {
        Write-Host "  Backend:  " -NoNewline
        Write-Host "✗ Not running" -ForegroundColor Red
    }
    
    if (Test-Port -Port 8080) {
        Write-Host "  Frontend: " -NoNewline
        Write-Host "✓ Running" -ForegroundColor Green -NoNewline
        Write-Host " on http://localhost:3080"
    } else {
        Write-Host "  Frontend: " -NoNewline
        Write-Host "✗ Not running" -ForegroundColor Red
    }
    
    Write-Host ""
    Write-Status "To stop servers: .\run.ps1 stop"
}

function Start-Servers {
    Write-Status "🚀 Starting Loco Platform Development Environment"
    Write-Host ""
    
    Set-Location $PSScriptRoot
    Stop-Servers
    Start-Sleep -Seconds 2
    
    Start-Backend
    Start-Sleep -Seconds 3
    Start-Frontend
    
    Wait-ForServers
    Show-Status
    
    Write-Status "Development environment is ready!"
}

function Show-Help {
    Write-Host "Loco Platform Development Server (PowerShell)"
    Write-Host ""
    Write-Host "Usage: .\run.ps1 [COMMAND]"
    Write-Host ""
    Write-Host "Commands:"
    Write-Host "  start     Start both frontend and backend servers (default)"
    Write-Host "  stop      Stop all servers"
    Write-Host "  restart   Restart all servers"
    Write-Host "  status    Show server status"
    Write-Host "  help      Show this help message"
    Write-Host ""
    Write-Host "Servers:"
    Write-Host "  Backend:  http://localhost:3070 (Axum/Rust)"
    Write-Host "  Frontend: http://localhost:3080 (Dioxus/WASM)"
}

# Main script logic
switch ($Command.ToLower()) {
    "start" { Start-Servers }
    "" { Start-Servers }
    "stop" { Stop-Servers }
    "restart" { 
        Stop-Servers
        Start-Sleep -Seconds 2
        Start-Servers
    }
    "status" { Show-Status }
    "help" { Show-Help }
    default {
        Write-Error "Unknown command: $Command"
        Write-Host ""
        Show-Help
        exit 1
    }
}