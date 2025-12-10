use crate::core::{AppError, AppResult, WallpaperType};
use crate::platform::WallpaperManager;
use log::{debug, error, info, warn};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::process::{Child, Command};
use tokio::sync::Mutex;
use async_trait::async_trait;

#[cfg(windows)]
use crate::platform::windows::window_manager::WindowManager;



/// Video wallpaper
pub struct VideoWallpaper {
    /// Video path
    path: PathBuf,
    
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
    
    /// Whether the video is playing
    is_playing: Arc<Mutex<bool>>,
    
    /// MPV process handle
    mpv_process: Arc<Mutex<Option<Child>>>,
    
    /// Window manager for desktop integration (Windows only)
    #[cfg(windows)]
    window_manager: Arc<Mutex<Option<WindowManager>>>,
}

impl VideoWallpaper {
    /// Create a new video wallpaper
    pub fn new<P: AsRef<Path>>(path: P, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            wallpaper_manager,
            is_playing: Arc::new(Mutex::new(false)),
            mpv_process: Arc::new(Mutex::new(None)),
            #[cfg(windows)]
            window_manager: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Check if MPV is available on the system
    fn check_mpv_available() -> bool {
        // Try multiple possible MPV locations
        let mpv_commands = vec![
            "mpv",                                    // Standard PATH
            "mpv.exe",                               // Windows with .exe
            "C:\\Program Files\\mpv\\mpv.exe",       // Common Windows install location
            "C:\\Program Files (x86)\\mpv\\mpv.exe", // 32-bit on 64-bit Windows
        ];

        for mpv_cmd in mpv_commands {
            match Command::new(mpv_cmd).arg("--version").output() {
                Ok(output) => {
                    if output.status.success() {
                        debug!("MPV is available at: {}", mpv_cmd);
                        return true;
                    } else {
                        debug!("MPV command failed at: {}", mpv_cmd);
                    }
                }
                Err(e) => {
                    debug!("MPV not found at {}: {}", mpv_cmd, e);
                }
            }
        }

        warn!("MPV not found in any standard locations");
        false
    }

    /// Get the MPV command path
    fn get_mpv_command() -> Result<String, AppError> {
        let mpv_commands = vec![
            "mpv",
            "mpv.exe",
            "C:\\Program Files\\mpv\\mpv.exe",
            "C:\\Program Files (x86)\\mpv\\mpv.exe",
        ];

        for mpv_cmd in mpv_commands {
            match Command::new(mpv_cmd).arg("--version").output() {
                Ok(output) => {
                    if output.status.success() {
                        debug!("Using MPV at: {}", mpv_cmd);
                        return Ok(mpv_cmd.to_string());
                    }
                }
                Err(_) => continue,
            }
        }

        Err(AppError::WallpaperError(
            "MPV is not installed or not available. Please install MPV from https://mpv.io/".to_string()
        ))
    }
    
    /// Start MPV with desktop integration
    async fn start_mpv(&self) -> Result<Child, AppError> {
        let mpv_command = Self::get_mpv_command()?;
        
        let mut cmd = Command::new(&mpv_command);
        
        // Basic MPV arguments for wallpaper mode (using most compatible options)
        cmd.args(&[
            "--loop-file=inf",           // Loop the video infinitely
            "--no-audio",                // Disable audio output
            "--no-border",               // Remove window border
            "--osd-level=0",             // Disable on-screen display
            "--quiet",                   // Reduce log output
            "--no-config",               // Don't load config files
        ]);

        // Add optional arguments that might not be supported in all versions
        let optional_args = vec![
            "--no-input-default-bindings", // Disable input handling
            "--no-input-cursor",         // Hide cursor
            "--hwdec=auto",              // Enable hardware decoding if available
            "--keepaspect=no",           // Don't maintain aspect ratio
            "--no-terminal",             // Don't use terminal
        ];

        // Try to add optional arguments, but don't fail if they're not supported
        for arg in optional_args {
            cmd.arg(arg);
        }
        
        // Platform-specific window integration
        #[cfg(windows)]
        {
            // Create a window manager and get the window handle
            let mut wm_guard = self.window_manager.lock().await;
            if wm_guard.is_none() {
                match WindowManager::new().create_wallpaper_window() {
                    Ok(window_hwnd) => {
                        let hwnd_str = format!("{}", window_hwnd.0);
                        
                        // Use the window ID for MPV
                        cmd.args(&[
                            "--wid", &hwnd_str,      // Embed in our window
                            "--no-keepaspect-window", // Don't maintain aspect ratio in window
                        ]);
                        
                        let wm = WindowManager::new();
                        *wm_guard = Some(wm);
                        debug!("Created wallpaper window with HWND: {}", hwnd_str);
                    }
                    Err(e) => {
                        warn!("Failed to create wallpaper window: {}. Using fullscreen mode instead.", e);
                        // Fallback to fullscreen mode
                        cmd.args(&[
                            "--fs",                  // Fullscreen
                            "--no-keepaspect",       // Don't maintain aspect ratio
                            "--ontop",               // Keep on top initially
                        ]);
                    }
                }
            } else {
                return Err(AppError::WallpaperError("Window manager already initialized".to_string()));
            }
        }
        
        #[cfg(not(windows))]
        {
            // On Linux, try to use fullscreen mode
            cmd.args(&[
                "--fs",                  // Fullscreen
                "--no-keepaspect",       // Don't maintain aspect ratio
            ]);
        }
        
        // Add the video file path
        cmd.arg(self.path.to_str().ok_or_else(|| {
            AppError::WallpaperError("Invalid video path".to_string())
        })?);
        
        info!("Starting MPV with command: {:?}", cmd);
        
        let child = cmd.spawn().map_err(|e| {
            error!("Failed to start MPV process: {}", e);
            AppError::WallpaperError(format!("Failed to start MPV: {}. Make sure MPV is installed and accessible.", e))
        })?;
        
        info!("MPV process started successfully for video: {}", self.path.display());
        Ok(child)
    }
}

#[async_trait]
impl super::Wallpaper for VideoWallpaper {
    fn get_type(&self) -> WallpaperType {
        WallpaperType::Video
    }
    
    fn get_path(&self) -> Option<&Path> {
        Some(&self.path)
    }
    
    async fn start(&self) -> AppResult<()> {
        debug!("Starting video wallpaper: {:?}", self.path);
        
        // Check if video file exists
        if !self.path.exists() {
            return Err(AppError::WallpaperError(format!(
                "Video file does not exist: {}", 
                self.path.display()
            )));
        }
        
        // Stop any existing process
        self.stop().await?;
        
        // Start MPV process
        let child = self.start_mpv().await?;
        
        // Store the process handle
        {
            let mut process = self.mpv_process.lock().await;
            *process = Some(child);
        }
        
        // Update playing state
        {
            let mut is_playing = self.is_playing.lock().await;
            *is_playing = true;
        }
        
        info!("Video wallpaper started successfully: {}", self.path.display());
        Ok(())
    }
    
    async fn stop(&self) -> AppResult<()> {
        debug!("Stopping video wallpaper");
        
        // Kill MPV process if running
        {
            let mut process = self.mpv_process.lock().await;
            if let Some(mut child) = process.take() {
                match child.kill() {
                    Ok(_) => {
                        debug!("MPV process terminated");
                        // Wait for the process to actually exit
                        let _ = child.wait();
                    }
                    Err(e) => {
                        warn!("Failed to kill MPV process: {}", e);
                        // Try to wait for it anyway
                        let _ = child.wait();
                    }
                }
            }
        }
        
        // Clean up window manager on Windows
        #[cfg(windows)]
        {
            let mut wm_guard = self.window_manager.lock().await;
            if let Some(wm) = wm_guard.take() {
                if let Err(e) = wm.hide_window() {
                    warn!("Failed to hide wallpaper window: {}", e);
                }
                debug!("Window manager cleaned up");
            }
        }
        
        // Update playing state
        {
            let mut is_playing = self.is_playing.lock().await;
            *is_playing = false;
        }
        
        info!("Video wallpaper stopped");
        Ok(())
    }
    
    async fn pause(&self) -> AppResult<()> {
        debug!("Pausing video wallpaper");
        
        // For now, we'll implement pause by stopping the video
        // A more sophisticated implementation would use MPV's IPC interface
        {
            let mut is_playing = self.is_playing.lock().await;
            if *is_playing {
                *is_playing = false;
                info!("Video wallpaper paused (stopped)");
            }
        }
        
        Ok(())
    }
    
    async fn resume(&self) -> AppResult<()> {
        debug!("Resuming video wallpaper");
        
        // For now, we'll implement resume by restarting the video
        // A more sophisticated implementation would use MPV's IPC interface
        let is_playing = {
            let is_playing = self.is_playing.lock().await;
            *is_playing
        };
        
        if !is_playing {
            self.start().await?;
            info!("Video wallpaper resumed (restarted)");
        }
        
        Ok(())
    }
} 