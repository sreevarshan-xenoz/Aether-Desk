#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

use crate::core::AppResult;
use std::path::Path;

/// Platform-specific wallpaper manager trait
pub trait WallpaperManager {
    /// Set a static image as wallpaper
    fn set_static_wallpaper(&self, path: &Path) -> AppResult<()>;
    
    /// Set a video as wallpaper
    fn set_video_wallpaper(&self, path: &Path) -> AppResult<()>;
    
    /// Set a web-based wallpaper
    fn set_web_wallpaper(&self, url: &str) -> AppResult<()>;
    
    /// Set a shader-based wallpaper
    fn set_shader_wallpaper(&self, shader_path: &Path) -> AppResult<()>;
    
    /// Set an audio-reactive wallpaper
    fn set_audio_wallpaper(&self, shader_path: &Path) -> AppResult<()>;
    
    /// Stop the current wallpaper
    fn stop_wallpaper(&self) -> AppResult<()>;
    
    /// Get the current wallpaper path
    fn get_current_wallpaper(&self) -> AppResult<Option<String>>;
} 