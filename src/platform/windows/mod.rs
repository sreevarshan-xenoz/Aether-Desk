use crate::core::{AppError, AppResult};
use crate::platform::WallpaperManager;
use log::{debug, error, info};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use winapi::um::winuser::{SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE};

/// Windows-specific wallpaper manager
pub struct WindowsWallpaperManager;

impl WindowsWallpaperManager {
    /// Create a new Windows wallpaper manager
    pub fn new() -> AppResult<Self> {
        Ok(Self)
    }
    
    /// Initialize the Windows wallpaper manager
    pub fn init() -> AppResult<()> {
        info!("Initializing Windows wallpaper manager");
        Ok(())
    }
}

impl WallpaperManager for WindowsWallpaperManager {
    fn set_static_wallpaper(&self, path: &Path) -> AppResult<()> {
        info!("Setting static wallpaper: {}", path.display());
        
        // Convert path to absolute path
        let path = path.canonicalize()?;
        
        // Use PowerShell to set the wallpaper
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                &format!(
                    "Add-Type -TypeDefinition @'
                    using System;
                    using System.Runtime.InteropServices;
                    public class Wallpaper {
                        [DllImport(\"user32.dll\", CharSet = CharSet.Auto)]
                        public static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);
                    }
                    '@;
                    [Wallpaper]::SystemParametersInfo(0x0014, 0, '{}', 0x01 -bor 0x02)",
                    path.to_string_lossy()
                ),
            ])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to set static wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()).into());
        }
        
        info!("Static wallpaper set successfully");
        Ok(())
    }
    
    fn set_video_wallpaper(&self, path: &Path) -> AppResult<()> {
        info!("Setting video wallpaper: {}", path.display());
        
        // Convert path to absolute path
        let path = path.canonicalize()?;
        
        // Use VLC to play the video as wallpaper
        let output = Command::new("vlc")
            .args(&[
                "--video-wallpaper",
                "--no-audio",
                "--loop",
                &path.to_string_lossy(),
            ])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to set video wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()).into());
        }
        
        info!("Video wallpaper set successfully");
        Ok(())
    }
    
    fn set_web_wallpaper(&self, url: &str) -> AppResult<()> {
        info!("Setting web wallpaper: {}", url);
        
        // Use a web browser to display the webpage as wallpaper
        let output = Command::new("start")
            .args(&["msedge", "--new-window", url])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to set web wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()).into());
        }
        
        info!("Web wallpaper set successfully");
        Ok(())
    }
    
    fn set_shader_wallpaper(&self, path: &Path) -> AppResult<()> {
        info!("Setting shader wallpaper: {}", path.display());
        
        // Convert path to absolute path
        let path = path.canonicalize()?;
        
        // Use a shader player to display the shader as wallpaper
        let output = Command::new("shadertoy")
            .args(&[&path.to_string_lossy()])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to set shader wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()).into());
        }
        
        info!("Shader wallpaper set successfully");
        Ok(())
    }
    
    fn set_audio_wallpaper(&self, path: &Path) -> AppResult<()> {
        info!("Setting audio wallpaper: {}", path.display());
        
        // Convert path to absolute path
        let path = path.canonicalize()?;
        
        // Use a shader player with audio visualization to display the shader as wallpaper
        let output = Command::new("shadertoy")
            .args(&["--audio", &path.to_string_lossy()])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to set audio wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()).into());
        }
        
        info!("Audio wallpaper set successfully");
        Ok(())
    }
    
    fn clear_wallpaper(&self) -> AppResult<()> {
        info!("Clearing wallpaper");
        
        // Use PowerShell to clear the wallpaper
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                "Add-Type -TypeDefinition @'
                using System;
                using System.Runtime.InteropServices;
                public class Wallpaper {
                    [DllImport(\"user32.dll\", CharSet = CharSet.Auto)]
                    public static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);
                }
                '@;
                [Wallpaper]::SystemParametersInfo(0x0014, 0, '', 0x01 -bor 0x02)",
            ])
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to clear wallpaper: {}", error);
            return Err(crate::core::AppError::WallpaperError(error.to_string()).into());
        }
        
        info!("Wallpaper cleared successfully");
        Ok(())
    }
} 