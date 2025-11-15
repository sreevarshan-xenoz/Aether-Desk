use crate::core::{AppError, AppResult, WallpaperType};
use crate::platform::WallpaperManager;
use log::{debug, error, info};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;

/// Web wallpaper
pub struct WebWallpaper {
    /// Web URL
    url: String,
    
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
    
    /// Whether the web wallpaper is active
    is_active: Arc<Mutex<bool>>,

    /// Browser process ID for control
    browser_pid: Arc<Mutex<Option<u32>>>,
}

impl WebWallpaper {
    /// Create a new web wallpaper
    pub fn new<S: Into<String>>(url: S, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            url: url.into(),
            wallpaper_manager,
            is_active: Arc::new(Mutex::new(false)),
            browser_pid: Arc::new(Mutex::new(None)),
        }
    }

    /// Find browser process ID for the current URL
    async fn find_browser_process(&self) -> AppResult<Option<u32>> {
        #[cfg(target_os = "windows")]
        {
            // Look for Edge/Chrome processes with the URL
            let output = Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!("Get-Process -Name msedge,chrome | Where-Object {{ $_.CommandLine -like '*{}*' }} | Select-Object -ExpandProperty Id", self.url)
                ])
                .output()?;

            if output.status.success() {
                let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !pid_str.is_empty() {
                    return Ok(Some(pid_str.parse::<u32>().map_err(|_| AppError::WallpaperError("Failed to parse browser PID".to_string()))?));
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Look for Firefox/Chrome processes with the URL
            for browser in &["firefox", "chrome", "chromium"] {
                let output = Command::new("pgrep")
                    .args(&["-f", &format!("{}.*{}", browser, self.url)])
                    .output()?;

                if output.status.success() {
                    let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !pid_str.is_empty() {
                        return Ok(Some(pid_str.parse::<u32>().map_err(|_| AppError::WallpaperError("Failed to parse browser PID".to_string()))?));
                    }
                }
            }
        }

        Ok(None)
    }
}

#[async_trait]
impl super::Wallpaper for WebWallpaper {
    fn get_type(&self) -> WallpaperType {
        WallpaperType::Web
    }
    
    fn get_path(&self) -> Option<&Path> {
        None
    }
    
    async fn start(&self) -> AppResult<()> {
        debug!("Starting web wallpaper: {}", self.url);
        
        // Set the wallpaper using the platform-specific manager
        self.wallpaper_manager.set_web_wallpaper(&self.url).await?;
        
        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = true;
        
        info!("Web wallpaper started");
        Ok(())
    }
    
    async fn stop(&self) -> AppResult<()> {
        debug!("Stopping web wallpaper");
        
        // Stop the wallpaper using the platform-specific manager
        self.wallpaper_manager.stop_wallpaper().await?;
        
        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = false;
        
        info!("Web wallpaper stopped");
        Ok(())
    }
    
    async fn pause(&self) -> AppResult<()> {
        debug!("Pausing web wallpaper");
        
        // TODO: Implement web wallpaper pausing
        error!("Web wallpaper pausing not implemented yet");
        Err(AppError::WallpaperError("Web wallpaper pausing not implemented yet".to_string()))
    }
    
    async fn resume(&self) -> AppResult<()> {
        debug!("Resuming web wallpaper");
        
        // TODO: Implement web wallpaper resuming
        error!("Web wallpaper resuming not implemented yet");
        Err(AppError::WallpaperError("Web wallpaper resuming not implemented yet".to_string()))
    }
} 