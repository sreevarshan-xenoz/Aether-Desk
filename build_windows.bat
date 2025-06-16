@echo off
echo Building Aether-Desk for Windows...
powershell -ExecutionPolicy Bypass -File build_windows.ps1
if %ERRORLEVEL% NEQ 0 (
    echo Build failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)
echo Build completed successfully!
echo.
echo To create an installer, you need to install NSIS (Nullsoft Scriptable Install System)
echo and run the following command:
echo.
echo makensis installer.nsi
echo.
pause 