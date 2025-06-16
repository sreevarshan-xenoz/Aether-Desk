# Build script for Aether-Desk Windows executable
# This script builds the application in release mode and packages it for distribution

# Set error action preference
$ErrorActionPreference = "Stop"

# Define paths
$projectRoot = $PSScriptRoot
$targetDir = Join-Path $projectRoot "target"
$releaseDir = Join-Path $targetDir "release"
$distDir = Join-Path $projectRoot "dist"
$windowsDistDir = Join-Path $distDir "windows"

# Create distribution directories if they don't exist
if (-not (Test-Path $distDir)) {
    New-Item -ItemType Directory -Path $distDir | Out-Null
    Write-Host "Created distribution directory: $distDir"
}

if (-not (Test-Path $windowsDistDir)) {
    New-Item -ItemType Directory -Path $windowsDistDir | Out-Null
    Write-Host "Created Windows distribution directory: $windowsDistDir"
}

# Build the project in release mode
Write-Host "Building Aether-Desk in release mode..."
cargo build --release

# Check if build was successful
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed with exit code $LASTEXITCODE"
    exit 1
}

# Copy executable and dependencies to distribution directory
$exePath = Join-Path $releaseDir "aether-desk.exe"
$distExePath = Join-Path $windowsDistDir "aether-desk.exe"

# Copy executable
Copy-Item -Path $exePath -Destination $distExePath -Force
Write-Host "Copied executable to: $distExePath"

# Copy README and license
Copy-Item -Path (Join-Path $projectRoot "README.md") -Destination $windowsDistDir -Force
Copy-Item -Path (Join-Path $projectRoot "LICENSE") -Destination $windowsDistDir -Force

# Create config and plugins directories
$configDir = Join-Path $windowsDistDir "config"
$pluginsDir = Join-Path $windowsDistDir "plugins"

if (-not (Test-Path $configDir)) {
    New-Item -ItemType Directory -Path $configDir | Out-Null
    Write-Host "Created config directory: $configDir"
}

if (-not (Test-Path $pluginsDir)) {
    New-Item -ItemType Directory -Path $pluginsDir | Out-Null
    Write-Host "Created plugins directory: $pluginsDir"
}

# Create a simple batch file to run the application
$batchContent = @"
@echo off
echo Starting Aether-Desk...
start "" "%~dp0aether-desk.exe"
"@

$batchPath = Join-Path $windowsDistDir "run-aether-desk.bat"
Set-Content -Path $batchPath -Value $batchContent
Write-Host "Created batch file: $batchPath"

# Create a ZIP archive for distribution
$zipPath = Join-Path $distDir "aether-desk-windows.zip"
if (Test-Path $zipPath) {
    Remove-Item -Path $zipPath -Force
}

Write-Host "Creating ZIP archive: $zipPath"
Compress-Archive -Path $windowsDistDir\* -DestinationPath $zipPath -Force

Write-Host "Build completed successfully!"
Write-Host "Windows executable is available at: $distExePath"
Write-Host "Distribution package is available at: $zipPath" 