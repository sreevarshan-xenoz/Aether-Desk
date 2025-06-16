@echo off
echo Building Aether-Desk for distribution...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo Build failed with error code %ERRORLEVEL%
    pause
    exit /b %ERRORLEVEL%
)
echo Build completed successfully!
echo.
echo Creating distribution package...
powershell -ExecutionPolicy Bypass -File build_windows.ps1
if %ERRORLEVEL% NEQ 0 (
    echo Distribution package creation failed with error code %ERRORLEVEL%
    pause
    exit /b %ERRORLEVEL%
)
echo Distribution package created successfully!
echo.
echo The Windows executable is available at: dist\windows\aether-desk.exe
echo The distribution package is available at: dist\aether-desk-windows.zip
echo.
pause 