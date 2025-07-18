use crate::core::{AppError, AppResult, WallpaperType};
use crate::platform::WallpaperManager;
use log::{debug, error, info};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Web wallpaper
pub struct WebWallpaper {
    /// Web URL
    url: String,
    
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
    
    /// Whether the web wallpaper is active
    is_active: Arc<Mutex<bool>>,
}

impl WebWallpaper {
    /// Create a new web wallpaper
    pub fn new<S: Into<String>>(url: S, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            url: url.into(),
            wallpaper_manager,
            is_active: Arc::new(Mutex::new(false)),
        }
    }
}

impl super::Wallpaper for WebWallpaper {
    fn get_type(&self) -> WallpaperType {
        WallpaperType::Web
    }
    
    fn get_path(&self) -> Option<&Path> {
        None
    }
    
    fn start(&self) -> AppResult<()> {
        debug!("Starting web wallpaper: {}", self.url);
        
        // Set the wallpaper using the platform-specific manager
        self.wallpaper_manager.set_web_wallpaper(&self.url)?;
        
        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = true;
        
        info!("Web wallpaper started");
        Ok(())
    }
    
    fn stop(&self) -> AppResult<()> {
        debug!("Stopping web wallpaper");
        
        // Stop the wallpaper using the platform-specific manager
        self.wallpaper_manager.stop_wallpaper()?;
        
        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = false;
        
        info!("Web wallpaper stopped");
        Ok(())
    }
    
    fn pause(&self) -> AppResult<()> {
        debug!("Pausing web wallpaper");
        
        // TODO: Implement web wallpaper pausing
        error!("Web wallpaper pausing not implemented yet");
        Err(AppError::Wallpaper("Web wallpaper pausing not implemented yet".to_string()))
    }
    
    fn resume(&self) -> AppResult<()> {
        debug!("Resuming web wallpaper");
        
        // TODO: Implement web wallpaper resuming
        error!("Web wallpaper resuming not implemented yet");
        Err(AppError::Wallpaper("Web wallpaper resuming not implemented yet".to_string()))
    }
} 