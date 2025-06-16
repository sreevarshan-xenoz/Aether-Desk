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
echo Creating icon...
powershell -ExecutionPolicy Bypass -File create_icon.ps1
if %ERRORLEVEL% NEQ 0 (
    echo Icon creation failed with error code %ERRORLEVEL%
    pause
    exit /b %ERRORLEVEL%
)
echo Icon created successfully!
echo.
echo Creating installer...
where makensis >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo NSIS (makensis) not found. Please install NSIS from https://nsis.sourceforge.io/Download
    echo and make sure it is in your PATH.
    pause
    exit /b 1
)
makensis installer.nsi
if %ERRORLEVEL% NEQ 0 (
    echo Installer creation failed with error code %ERRORLEVEL%
    pause
    exit /b %ERRORLEVEL%
)
echo Installer created successfully!
echo.
echo The Windows executable is available at: dist\windows\aether-desk.exe
echo The distribution package is available at: dist\aether-desk-windows.zip
echo The installer is available at: Aether-Desk-Setup.exe
echo.
pause 