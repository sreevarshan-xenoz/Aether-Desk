mod windows;
mod linux;
mod hyprland;

use crate::core::AppResult;
use std::sync::Arc;

/// Platform-specific wallpaper manager
pub trait WallpaperManager: Send + Sync {
    /// Set a static wallpaper
    fn set_static_wallpaper(&self, path: &std::path::Path) -> AppResult<()>;
    
    /// Set a video wallpaper
    fn set_video_wallpaper(&self, path: &std::path::Path) -> AppResult<()>;
    
    /// Set a web wallpaper
    fn set_web_wallpaper(&self, url: &str) -> AppResult<()>;
    
    /// Set a shader wallpaper
    fn set_shader_wallpaper(&self, path: &std::path::Path) -> AppResult<()>;
    
    /// Set an audio wallpaper
    fn set_audio_wallpaper(&self, path: &std::path::Path) -> AppResult<()>;
    
    /// Clear the current wallpaper
    fn clear_wallpaper(&self) -> AppResult<()>;
    
    /// Stop the current wallpaper
    fn stop_wallpaper(&self) -> AppResult<()>;
}

/// Create a platform-specific wallpaper manager
pub fn create_wallpaper_manager() -> AppResult<Arc<dyn WallpaperManager + Send + Sync>> {
    #[cfg(target_os = "windows")]
    {
        Ok(Arc::new(windows::WindowsWallpaperManager::new()?))
    }
    
    #[cfg(target_os = "linux")]
    {
        // Check if running on Hyprland
        if hyprland::is_hyprland() {
            Ok(hyprland::create_hyprland_wallpaper_manager())
        } else {
            Ok(Arc::new(linux::LinuxWallpaperManager::new()?))
        }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Err(crate::core::AppError::UnsupportedPlatform.into())
    }
} 