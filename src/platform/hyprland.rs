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
    
async fn set_video_wallpaper(&self, path: &Path) -> AppResult<()> {
        // Convert path to string
        let path_str = path.to_string_lossy().to_string();

        // Launch VLC in wallpaper mode for video wallpapers
        let output = Command::new("vlc")
            .args(&[
                "--wallpaper-mode",
                "--no-video-title-show",
                "--loop",
                "--no-audio", // Audio can be handled separately if needed
                &path_str,
            ])
            .output()
            .map_err(|e| format!("Failed to execute VLC: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to set video wallpaper: {}", error).into());
        }

        Ok(())
    }
    
async fn set_web_wallpaper(&self, url: &str) -> AppResult<()> {
        // Launch Firefox in kiosk mode for web wallpapers
        let output = Command::new("firefox")
            .args(&[
                "--kiosk",
                url,
            ])
            .output()
            .map_err(|e| format!("Failed to execute Firefox: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to set web wallpaper: {}", error).into());
        }

        Ok(())
    }
    
async fn set_shader_wallpaper(&self, path: &Path) -> AppResult<()> {
        // Convert path to string
        let path_str = path.to_string_lossy().to_string();

        // Try glslviewer first for shader wallpapers
        let output = Command::new("glslviewer")
            .args(&[
                "--fullscreen",
                &path_str,
            ])
            .output()
            .map_err(|e| format!("Failed to execute glslviewer: {}", e))?;

        if output.status.success() {
            return Ok(());
        }

        // Fallback to shadertoy if glslviewer fails
        let output = Command::new("shadertoy")
            .args(&[&path_str])
            .output()
            .map_err(|e| format!("Failed to execute shadertoy: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to set shader wallpaper: {}", error).into());
        }

        Ok(())
    }
    
async fn set_audio_wallpaper(&self, path: &Path) -> AppResult<()> {
        // Convert path to string
        let path_str = path.to_string_lossy().to_string();

        // Try cava first for audio wallpapers
        let output = Command::new("cava")
            .args(&[
                "--fullscreen",
                &path_str,
            ])
            .output()
            .map_err(|e| format!("Failed to execute cava: {}", e))?;

        if output.status.success() {
            return Ok(());
        }

        // Fallback to shadertoy with audio support
        let output = Command::new("shadertoy")
            .args(&[
                "--audio",
                &path_str,
            ])
            .output()
            .map_err(|e| format!("Failed to execute shadertoy: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to set audio wallpaper: {}", error).into());
        }

        Ok(())
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
    
    async fn get_current_wallpaper(&self) -> AppResult<Option<std::path::PathBuf>> {
        // For initial compilation, return placeholder value
        Ok(None)
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

#[allow(dead_code)]
pub fn is_hyprland() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP").map_or(false, |v| v.to_lowercase().contains("hyprland"))
}

#[allow(dead_code)]
pub fn create_hyprland_wallpaper_manager() -> Arc<dyn WallpaperManager + Send + Sync> {
    Arc::new(HyprlandWallpaperManager)
} 