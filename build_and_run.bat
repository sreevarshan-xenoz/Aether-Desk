@echo off
echo Building Aether-Desk...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo Build failed with error code %ERRORLEVEL%
    pause
    exit /b %ERRORLEVEL%
)
echo Build completed successfully!
echo.
echo Starting Aether-Desk...
start "" "%~dp0target\release\aether-desk.exe" 