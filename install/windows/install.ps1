# install/windows/install.ps1 - Install as Windows Service

$ServiceName = "DeviceAgent"
$BinaryPath = "C:\Program Files\DeviceAgent\device-agent.exe"
$InstallDir = "C:\Program Files\DeviceAgent"

Write-Host "=================================" -ForegroundColor Cyan
Write-Host "Device Agent - Windows Installation" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

# Check admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "Error: Please run as Administrator" -ForegroundColor Red
    exit 1
}

# Build
Write-Host "Step 1: Building release binary..." -ForegroundColor Yellow
cargo build --release

# Create directories
Write-Host "Step 2: Creating directories..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
New-Item -ItemType Directory -Force -Path "$InstallDir\data" | Out-Null
New-Item -ItemType Directory -Force -Path "$InstallDir\logs" | Out-Null

# Copy files
Copy-Item "target\release\device-agent.exe" $BinaryPath -Force
Copy-Item "config.toml" "$InstallDir\config.toml" -Force

# Update config paths
(Get-Content "$InstallDir\config.toml") -replace 'output_directory = "./data"', "output_directory = `"$InstallDir\data`"" | Set-Content "$InstallDir\config.toml"
(Get-Content "$InstallDir\config.toml") -replace 'log_directory = "./logs"', "log_directory = `"$InstallDir\logs`"" | Set-Content "$InstallDir\config.toml"

# Install service
Write-Host "Step 3: Installing Windows Service..." -ForegroundColor Yellow
sc.exe create $ServiceName binPath= $BinaryPath start= auto
sc.exe description $ServiceName "Device monitoring and management agent"
sc.exe start $ServiceName

Write-Host ""
Write-Host "=================================" -ForegroundColor Green
Write-Host "Installation Complete!" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green
Write-Host ""
Write-Host "Service Commands:"
Write-Host "  Start:   sc.exe start $ServiceName"
Write-Host "  Stop:    sc.exe stop $ServiceName"
Write-Host "  Status:  sc.exe query $ServiceName"
Write-Host ""