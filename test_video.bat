@echo off
echo Testing Aether-Desk Video Wallpaper Implementation
echo.
echo This will test if the video wallpaper functionality works.
echo Make sure you have MPV installed and a test video file ready.
echo.
echo Building the application...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo Build failed!
    pause
    exit /b 1
)

echo.
echo Build successful! You can now test the video wallpaper feature.
echo.
echo To test:
echo 1. Run the application: target\release\aether-desk.exe
echo 2. Go to the Wallpaper tab
echo 3. Select "Video" as wallpaper type
echo 4. Browse for an MP4 video file
echo 5. Click "Apply"
echo.
echo The video should appear as your desktop wallpaper behind icons.
echo.
pause