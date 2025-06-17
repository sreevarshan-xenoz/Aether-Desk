use crate::core::AppResult;
use crate::platform::WallpaperManager;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

/// Hyprland-specific wallpaper manager
pub struct HyprlandWallpaperManager;

impl WallpaperManager for HyprlandWallpaperManager {
    fn set_static_wallpaper(&self, path: &Path) -> AppResult<()> {
        // Convert path to string
        let path_str = path.to_string_lossy().to_string();
        
        // Use hyprctl to set the wallpaper
        let output = Command::new("hyprctl")
            .args(&["hyprpaper", "wallpaper", "eDP-1,", &path_str])
            .output()
            .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to set wallpaper: {}", error).into());
        }
        
        Ok(())
    }
    
    fn set_video_wallpaper(&self, _path: &Path) -> AppResult<()> {
        // TODO: Implement video wallpaper support for Hyprland
        Err("Video wallpapers not yet supported for Hyprland".into())
    }
    
    fn set_web_wallpaper(&self, _url: &str) -> AppResult<()> {
        // TODO: Implement web wallpaper support for Hyprland
        Err("Web wallpapers not yet supported for Hyprland".into())
    }
    
    fn set_shader_wallpaper(&self, _path: &Path) -> AppResult<()> {
        // TODO: Implement shader wallpaper support for Hyprland
        Err("Shader wallpapers not yet supported for Hyprland".into())
    }
    
    fn set_audio_wallpaper(&self, _path: &Path) -> AppResult<()> {
        // TODO: Implement audio wallpaper support for Hyprland
        Err("Audio wallpapers not yet supported for Hyprland".into())
    }
    
    fn clear_wallpaper(&self) -> AppResult<()> {
        // Use hyprctl to clear the wallpaper
        let output = Command::new("hyprctl")
            .args(&["hyprpaper", "unload", "all"])
            .output()
            .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to clear wallpaper: {}", error).into());
        }
        
        Ok(())
    }
}

pub fn is_hyprland() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP").map_or(false, |v| v.to_lowercase().contains("hyprland"))
}

pub fn create_hyprland_wallpaper_manager() -> Arc<dyn WallpaperManager + Send + Sync> {
    Arc::new(HyprlandWallpaperManager)
} 