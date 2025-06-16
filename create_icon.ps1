# This script creates a simple icon for the Aether-Desk application
# It requires the .NET Framework to be installed

# Create assets directory if it doesn't exist
$assetsDir = Join-Path $PSScriptRoot "assets"
if (-not (Test-Path $assetsDir)) {
    New-Item -ItemType Directory -Path $assetsDir | Out-Null
    Write-Host "Created assets directory: $assetsDir"
}

# Create a simple icon using .NET
$iconPath = Join-Path $assetsDir "icon.ico"

# Check if the icon already exists
if (Test-Path $iconPath) {
    Write-Host "Icon already exists at: $iconPath"
    exit 0
}

# Create a simple icon using .NET
$assembly = [System.Reflection.Assembly]::LoadWithPartialName("System.Drawing")
if ($null -eq $assembly) {
    Write-Error "Failed to load System.Drawing assembly. Make sure .NET Framework is installed."
    exit 1
}

try {
    # Create a bitmap
    $bitmap = New-Object System.Drawing.Bitmap 256, 256
    
    # Create a graphics object
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    
    # Fill the background
    $graphics.Clear([System.Drawing.Color]::FromArgb(255, 41, 128, 185))
    
    # Draw a simple design
    $pen = New-Object System.Drawing.Pen([System.Drawing.Color]::White, 10)
    $graphics.DrawEllipse($pen, 50, 50, 156, 156)
    $graphics.DrawLine($pen, 128, 50, 128, 206)
    $graphics.DrawLine($pen, 50, 128, 206, 128)
    
    # Save the bitmap as an icon
    $icon = [System.Drawing.Icon]::FromHandle($bitmap.GetHicon())
    $fileStream = New-Object System.IO.FileStream($iconPath, [System.IO.FileMode]::Create)
    $icon.Save($fileStream)
    $fileStream.Close()
    
    # Clean up
    $icon.Dispose()
    $graphics.Dispose()
    $bitmap.Dispose()
    
    Write-Host "Created icon at: $iconPath"
} catch {
    Write-Error "Failed to create icon: $_"
    exit 1
} 