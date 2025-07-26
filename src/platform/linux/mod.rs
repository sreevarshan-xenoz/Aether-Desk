use async_trait::async_trait;
use crate::core::{AppError, AppResult};
use crate::platform::WallpaperManager;
use log::{debug, error, info};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Linux wallpaper manager
pub struct LinuxWallpaperManager {
    /// Current wallpaper path
    current_wallpaper: Arc<Mutex<Option<String>>>,
    
    /// Desktop environment
    desktop_env: String,
}

#[allow(dead_code)]
impl LinuxWallpaperManager {
    /// Create a new Linux wallpaper manager
    pub fn new() -> AppResult<Self> {
        let desktop_env = Self::detect_desktop_environment();
        info!("Detected desktop environment: {}", desktop_env);
        
        Ok(Self {
            current_wallpaper: Arc::new(Mutex::new(None)),
            desktop_env,
        })
    }
    
    /// Initialize the Linux wallpaper manager
    pub fn init() -> AppResult<()> {
        info!("Initializing Linux wallpaper manager");
        Ok(())
    }
    
    /// Detect the current desktop environment
    fn detect_desktop_environment() -> String {
        // Check for common desktop environment variables
        if let Ok(env) = std::env::var("XDG_CURRENT_DESKTOP") {
            return env;
        }
        
        if let Ok(env) = std::env::var("DESKTOP_SESSION") {
            return env;
        }
        
        if let Ok(_env) = std::env::var("GNOME_DESKTOP_SESSION_ID") {
            return "GNOME".to_string();
        }
        
        if let Ok(_env) = std::env::var("KDE_FULL_SESSION") {
            return "KDE".to_string();
        }
        
        // Default to generic
        "generic".to_string()
    }
    
    /// Set wallpaper using feh (works on most X11 environments)
    fn set_wallpaper_with_feh(&self, path: &Path) -> AppResult<()> {
        let path_str = path.to_string_lossy().to_string();
        debug!("Setting wallpaper with feh: {}", path_str);
        
        let output = Command::new("feh")
            .args(&["--bg-fill", &path_str])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::PlatformError(format!("feh failed: {}", error)));
        }
        
        Ok(())
    }
    
    /// Set wallpaper using gsettings (works on GNOME)
    fn set_wallpaper_with_gsettings(&self, path: &Path) -> AppResult<()> {
        let path_str = path.to_string_lossy().to_string();
        debug!("Setting wallpaper with gsettings: {}", path_str);
        
        let output = Command::new("gsettings")
            .args(&["set", "org.gnome.desktop.background", "picture-uri", &format!("file://{}", path_str)])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::PlatformError(format!("gsettings failed: {}", error)));
        }
        
        Ok(())
    }
    
    /// Set wallpaper using xfconf-query (works on XFCE)
    fn set_wallpaper_with_xfconf(&self, path: &Path) -> AppResult<()> {
        let path_str = path.to_string_lossy().to_string();
        debug!("Setting wallpaper with xfconf-query: {}", path_str);
        
        let output = Command::new("xfconf-query")
            .args(&["-c", "xfce4-desktop", "-p", "/backdrop/screen0/monitor0/image-path", "-s", &path_str])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::PlatformError(format!("xfconf-query failed: {}", error)));
        }
        
        Ok(())
    }
    
    /// Set wallpaper using swww (works on Wayland with Hyprland)
    fn set_wallpaper_with_swww(&self, path: &Path) -> AppResult<()> {
        let path_str = path.to_string_lossy().to_string();
        debug!("Setting wallpaper with swww: {}", path_str);
        
        let output = Command::new("swww")
            .args(&["img", &path_str])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::PlatformError(format!("swww failed: {}", error)));
        }
        
        Ok(())
    }
}

#[async_trait]
impl WallpaperManager for LinuxWallpaperManager {
    async fn set_static_wallpaper(&self, path: &Path) -> AppResult<()> {
        info!("Setting static wallpaper: {}", path.display());
        
        // Convert path to absolute path
        let path = path.canonicalize()?;
        
        // Try different methods to set the wallpaper
        let mut success = false;
        
        // Try using gsettings (GNOME)
        let output = Command::new("gsettings")
            .args(&["set", "org.gnome.desktop.background", "picture-uri", &format!("file://{}", path.to_string_lossy().to_string())])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                success = true;
                info!("Static wallpaper set successfully using gsettings");
            }
        }
        
        // Try using feh
        if !success {
            let output = Command::new("feh")
                .args(&["--bg-fill", &path.to_string_lossy().to_string()])
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    success = true;
                    info!("Static wallpaper set successfully using feh");
                }
            }
        }
        
        // Try using nitrogen
        if !success {
            let output = Command::new("nitrogen")
                .args(&["--set-zoom-fill", &path.to_string_lossy().to_string()])
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    success = true;
                    info!("Static wallpaper set successfully using nitrogen");
                }
            }
        }
        
        if !success {
            error!("Failed to set static wallpaper using any method");
            return Err(crate::core::AppError::WallpaperError("Failed to set static wallpaper".to_string()));
        }
        
        // Update current wallpaper
        let mut current = self.current_wallpaper.lock().await;
        *current = Some(path.to_string_lossy().to_string());
        
        Ok(())
    }
    
    async fn set_video_wallpaper(&self, path: &Path) -> AppResult<()> {
        info!("Setting video wallpaper: {}", path.display());
        
        // Convert path to absolute path
        let path = path.canonicalize()?;
        
        // Use VLC to play the video as wallpaper
        let output = Command::new("vlc")
            .args(&[
                "--video-wallpaper",
                "--no-audio",
                "--loop",
                &path.to_string_lossy().to_string(),
            ])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to set video wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()));
        }
        
        info!("Video wallpaper set successfully");
        Ok(())
    }
    
    async fn set_web_wallpaper(&self, url: &str) -> AppResult<()> {
        info!("Setting web wallpaper: {}", url);
        
        // Use a web browser to display the webpage as wallpaper
        let output = Command::new("firefox")
            .args(&["--new-window", url])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to set web wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()));
        }
        
        info!("Web wallpaper set successfully");
        Ok(())
    }
    
    async fn set_shader_wallpaper(&self, path: &Path) -> AppResult<()> {
        info!("Setting shader wallpaper: {}", path.display());
        
        // Convert path to absolute path
        let path = path.canonicalize()?;
        
        // Use a shader player to display the shader as wallpaper
        let output = Command::new("shadertoy")
            .args(&[&path.to_string_lossy().to_string()])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to set shader wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()));
        }
        
        info!("Shader wallpaper set successfully");
        Ok(())
    }
    
    async fn set_audio_wallpaper(&self, path: &Path) -> AppResult<()> {
        info!("Setting audio wallpaper: {}", path.display());
        
        // Convert path to absolute path
        let path = path.canonicalize()?;
        
        // Use a shader player with audio visualization to display the shader as wallpaper
        let output = Command::new("shadertoy")
            .args(&["--audio", &path.to_string_lossy().to_string()])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to set audio wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()));
        }
        
        info!("Audio wallpaper set successfully");
        Ok(())
    }
    
    async fn clear_wallpaper(&self) -> AppResult<()> {
        info!("Clearing wallpaper");
        
        // Try different methods to clear the wallpaper
        let mut success = false;
        
        // Try using gsettings (GNOME)
        let output = Command::new("gsettings")
            .args(&["set", "org.gnome.desktop.background", "picture-uri", ""])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                success = true;
                info!("Wallpaper cleared successfully using gsettings");
            }
        }
        
        // Try using feh
        if !success {
            let output = Command::new("feh")
                .args(&["--bg-fill", "--no-fehbg"])
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    success = true;
                    info!("Wallpaper cleared successfully using feh");
                }
            }
        }
        
        // Try using nitrogen
        if !success {
            let output = Command::new("nitrogen")
                .args(&["--restore"])
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    success = true;
                    info!("Wallpaper cleared successfully using nitrogen");
                }
            }
        }
        
        if !success {
            error!("Failed to clear wallpaper using any method");
            return Err(crate::core::AppError::WallpaperError("Failed to clear wallpaper".to_string()));
        }
        
        // Clear current wallpaper
        let mut current = self.current_wallpaper.lock().await;
        *current = None;
        
        Ok(())
    }
    
    async fn get_current_wallpaper(&self) -> AppResult<Option<std::path::PathBuf>> {
        let current = self.current_wallpaper.lock().await;
        Ok(current.as_ref().map(|path| std::path::PathBuf::from(path)))
    }
    
    async fn stop_wallpaper(&self) -> AppResult<()> {
        info!("Stopping wallpaper");
        self.clear_wallpaper().await
    }
}
