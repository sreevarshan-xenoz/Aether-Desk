use crate::core::AppResult;
use crate::platform::WallpaperManager;
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

/// Hyprland-specific wallpaper manager
pub struct HyprlandWallpaperManager;

#[async_trait]
impl WallpaperManager for HyprlandWallpaperManager {
async fn set_static_wallpaper(&self, path: &Path) -> AppResult<()> {
        // Convert path to string
        let path_str = path.to_string_lossy().to_string();
        
        // Get list of monitors
        let monitors = self.get_monitors()?;
        
        if monitors.is_empty() {
            return Err("No monitors detected".into());
        }
        
        // Set wallpaper for each monitor
        for monitor in monitors {
            let output = Command::new("hyprctl")
                .args(&["hyprpaper", "wallpaper", &format!("{},", monitor), &path_str])
                .output()
                .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;
            
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to set wallpaper for monitor {}: {}", monitor, error).into());
            }
        }
        
        Ok(())
    }
    
async fn set_video_wallpaper(&self, _path: &Path) -> AppResult<()> {
        // TODO: Implement video wallpaper support for Hyprland
        Err("Video wallpapers not yet supported for Hyprland".into())
    }
    
async fn set_web_wallpaper(&self, _url: &str) -> AppResult<()> {
        // TODO: Implement web wallpaper support for Hyprland
        Err("Web wallpapers not yet supported for Hyprland".into())
    }
    
async fn set_shader_wallpaper(&self, _path: &Path) -> AppResult<()> {
        // TODO: Implement shader wallpaper support for Hyprland
        Err("Shader wallpapers not yet supported for Hyprland".into())
    }
    
async fn set_audio_wallpaper(&self, _path: &Path) -> AppResult<()> {
        // TODO: Implement audio wallpaper support for Hyprland
        Err("Audio wallpapers not yet supported for Hyprland".into())
    }
    
    async fn clear_wallpaper(&self) -> AppResult<()> {
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
    
    async fn stop_wallpaper(&self) -> AppResult<()> {
        // For Hyprland, stopping wallpaper is the same as clearing it
        self.clear_wallpaper().await
    }
}

impl HyprlandWallpaperManager {
    /// Get a list of available monitors
    fn get_monitors(&self) -> AppResult<Vec<String>> {
        let output = Command::new("hyprctl")
            .args(&["monitors"])
            .output()
            .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to get monitors: {}", error).into());
        }
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut monitors = Vec::new();
        
        // Parse the output to extract monitor names
        for line in output_str.lines() {
            if line.contains("Monitor") && line.contains("(") {
                // Extract monitor name from line like "Monitor eDP-1 (ID 0): 1920x1080 @ 60.000000 Hz"
                if let Some(start) = line.find("Monitor ") {
                    let start = start + 8; // Skip "Monitor "
                    if let Some(end) = line[start..].find(" ") {
                        let monitor_name = line[start..start+end].to_string();
                        monitors.push(monitor_name);
                    }
                }
            }
        }
        
        Ok(monitors)
    }
}

pub fn is_hyprland() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP").map_or(false, |v| v.to_lowercase().contains("hyprland"))
}

pub fn create_hyprland_wallpaper_manager() -> Arc<dyn WallpaperManager + Send + Sync> {
    Arc::new(HyprlandWallpaperManager)
} 