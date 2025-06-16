use crate::core::{AppError, AppResult, WallpaperType};
use crate::platform::WallpaperManager;
use log::{debug, info};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Static wallpaper
pub struct StaticWallpaper {
    /// Wallpaper path
    path: PathBuf,
    
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
}

impl StaticWallpaper {
    /// Create a new static wallpaper
    pub fn new<P: AsRef<Path>>(path: P, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            wallpaper_manager,
        }
    }
}

impl super::Wallpaper for StaticWallpaper {
    fn get_type(&self) -> WallpaperType {
        WallpaperType::Static
    }
    
    fn get_path(&self) -> Option<&Path> {
        Some(&self.path)
    }
    
    fn start(&self) -> AppResult<()> {
        debug!("Starting static wallpaper: {:?}", self.path);
        
        // Set the wallpaper using the platform-specific manager
        self.wallpaper_manager.set_static_wallpaper(&self.path)?;
        
        info!("Static wallpaper started");
        Ok(())
    }
    
    fn stop(&self) -> AppResult<()> {
        debug!("Stopping static wallpaper");
        
        // Stop the wallpaper using the platform-specific manager
        self.wallpaper_manager.stop_wallpaper()?;
        
        info!("Static wallpaper stopped");
        Ok(())
    }
    
    fn pause(&self) -> AppResult<()> {
        // Static wallpapers don't need to be paused
        Ok(())
    }
    
    fn resume(&self) -> AppResult<()> {
        // Static wallpapers don't need to be resumed
        Ok(())
    }
} 