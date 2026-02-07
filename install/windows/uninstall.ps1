cat > install/windows/uninstall.ps1 << 'EOF'
# install/windows/uninstall.ps1 - Uninstall device-agent service

$ServiceName = "DeviceAgent"
$BinaryPath = "C:\Program Files\DeviceAgent\device-agent.exe"
$InstallDir = "C:\Program Files\DeviceAgent"

Write-Host "======================================" -ForegroundColor Cyan
Write-Host "Device Agent - Windows Uninstallation" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "Error: Please run as Administrator" -ForegroundColor Red
    Write-Host "Right-click PowerShell and select 'Run as Administrator'" -ForegroundColor Yellow
    exit 1
}

# Step 1: Check if service exists
Write-Host "Step 1: Checking service status..." -ForegroundColor Yellow
$service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue

if ($service) {
    # Step 2: Stop service
    Write-Host "Step 2: Stopping service..." -ForegroundColor Yellow
    if ($service.Status -eq 'Running') {
        Stop-Service -Name $ServiceName -Force
        Start-Sleep -Seconds 2
        Write-Host "√ Service stopped" -ForegroundColor Green
    } else {
        Write-Host "! Service not running" -ForegroundColor Yellow
    }
    
    # Step 3: Delete service
    Write-Host "Step 3: Removing service..." -ForegroundColor Yellow
    sc.exe delete $ServiceName | Out-Null
    Write-Host "√ Service removed" -ForegroundColor Green
} else {
    Write-Host "! Service not found" -ForegroundColor Yellow
}

# Step 4: Ask about data removal
Write-Host ""
Write-Host "Data and configuration files are located at:" -ForegroundColor Yellow
Write-Host "  $InstallDir" -ForegroundColor Cyan
Write-Host ""

$response = Read-Host "Do you want to remove all data and configuration? (y/N)"

if ($response -eq 'y' -or $response -eq 'Y') {
    Write-Host "Step 4: Removing all files..." -ForegroundColor Yellow
    
    if (Test-Path $InstallDir) {
        Remove-Item -Path $InstallDir -Recurse -Force -ErrorAction SilentlyContinue
        Write-Host "√ All files removed" -ForegroundColor Green
    } else {
        Write-Host "! Installation directory not found" -ForegroundColor Yellow
    }
} else {
    Write-Host "Data preserved at: $InstallDir" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "To manually remove data later:" -ForegroundColor Cyan
    Write-Host "  Remove-Item -Path '$InstallDir' -Recurse -Force" -ForegroundColor White
}

Write-Host ""
Write-Host "======================================" -ForegroundColor Green
Write-Host "Uninstallation Complete!" -ForegroundColor Green
Write-Host "======================================" -ForegroundColor Green
Write-Host ""
EOF