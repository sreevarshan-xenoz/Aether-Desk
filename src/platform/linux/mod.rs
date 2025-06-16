use crate::core::{AppError, AppResult};
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

impl LinuxWallpaperManager {
    /// Create a new Linux wallpaper manager
    pub fn new() -> Self {
        let desktop_env = Self::detect_desktop_environment();
        info!("Detected desktop environment: {}", desktop_env);
        
        Self {
            current_wallpaper: Arc::new(Mutex::new(None)),
            desktop_env,
        }
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
        
        if let Ok(env) = std::env::var("GNOME_DESKTOP_SESSION_ID") {
            return "GNOME".to_string();
        }
        
        if let Ok(env) = std::env::var("KDE_FULL_SESSION") {
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
            return Err(AppError::Platform(format!("feh failed: {}", error)));
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
            return Err(AppError::Platform(format!("gsettings failed: {}", error)));
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
            return Err(AppError::Platform(format!("xfconf-query failed: {}", error)));
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
            return Err(AppError::Platform(format!("swww failed: {}", error)));
        }
        
        Ok(())
    }
}

impl super::WallpaperManager for LinuxWallpaperManager {
    fn set_static_wallpaper(&self, path: &Path) -> AppResult<()> {
        let path_str = path.to_string_lossy().to_string();
        debug!("Setting static wallpaper: {}", path_str);
        
        // Try different methods based on desktop environment
        match self.desktop_env.to_lowercase().as_str() {
            "gnome" | "ubuntu" | "pop" => {
                self.set_wallpaper_with_gsettings(path)?;
            },
            "xfce" => {
                self.set_wallpaper_with_xfconf(path)?;
            },
            "hyprland" | "sway" => {
                self.set_wallpaper_with_swww(path)?;
            },
            _ => {
                // Try feh as a fallback
                self.set_wallpaper_with_feh(path)?;
            }
        }
        
        // Update current wallpaper
        let mut current = self.current_wallpaper.lock().await;
        *current = Some(path_str);
        
        info!("Static wallpaper set successfully");
        Ok(())
    }
    
    fn set_video_wallpaper(&self, path: &Path) -> AppResult<()> {
        // TODO: Implement video wallpaper using mpv or other methods
        error!("Video wallpapers not implemented yet");
        Err(AppError::Platform("Video wallpapers not implemented yet".to_string()))
    }
    
    fn set_web_wallpaper(&self, url: &str) -> AppResult<()> {
        // TODO: Implement web wallpaper using WebKit or other methods
        error!("Web wallpapers not implemented yet");
        Err(AppError::Platform("Web wallpapers not implemented yet".to_string()))
    }
    
    fn set_shader_wallpaper(&self, shader_path: &Path) -> AppResult<()> {
        // TODO: Implement shader wallpaper using OpenGL
        error!("Shader wallpapers not implemented yet");
        Err(AppError::Platform("Shader wallpapers not implemented yet".to_string()))
    }
    
    fn set_audio_wallpaper(&self, shader_path: &Path) -> AppResult<()> {
        // TODO: Implement audio-reactive wallpaper
        error!("Audio-reactive wallpapers not implemented yet");
        Err(AppError::Platform("Audio-reactive wallpapers not implemented yet".to_string()))
    }
    
    fn stop_wallpaper(&self) -> AppResult<()> {
        debug!("Stopping current wallpaper");
        
        // Clear current wallpaper
        let mut current = self.current_wallpaper.lock().await;
        *current = None;
        
        info!("Wallpaper stopped");
        Ok(())
    }
    
    fn get_current_wallpaper(&self) -> AppResult<Option<String>> {
        let current = self.current_wallpaper.lock().await;
        Ok(current.clone())
    }
} 