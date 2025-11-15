use crate::core::{AppError, AppResult, WallpaperType};
use crate::platform::WallpaperManager;
use log::{debug, error, info};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;

/// Audio wallpaper
pub struct AudioWallpaper {
    /// Shader path
    path: PathBuf,
    
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
    
    /// Whether the audio wallpaper is active
    is_active: Arc<Mutex<bool>>,
}

impl AudioWallpaper {
    /// Create a new audio wallpaper
    pub fn new<P: AsRef<Path>>(path: P, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            wallpaper_manager,
            is_active: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait]
impl super::Wallpaper for AudioWallpaper {
    fn get_type(&self) -> WallpaperType {
        WallpaperType::Audio
    }
    
    fn get_path(&self) -> Option<&Path> {
        Some(&self.path)
    }
    
    async fn start(&self) -> AppResult<()> {
        debug!("Starting audio wallpaper: {:?}", self.path);
        
        // Set the wallpaper using the platform-specific manager
        self.wallpaper_manager.set_audio_wallpaper(&self.path).await?;
        
        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = true;
        
        info!("Audio wallpaper started");
        Ok(())
    }
    
    async fn stop(&self) -> AppResult<()> {
        debug!("Stopping audio wallpaper");
        
        // Stop the wallpaper using the platform-specific manager
        self.wallpaper_manager.stop_wallpaper().await?;
        
        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = false;
        
        info!("Audio wallpaper stopped");
        Ok(())
    }
    
    async fn pause(&self) -> AppResult<()> {
        debug!("Pausing audio wallpaper");
        
        // TODO: Implement audio wallpaper pausing
        error!("Audio wallpaper pausing not implemented yet");
        Err(AppError::WallpaperError("Audio wallpaper pausing not implemented yet".to_string()))
    }
    
    async fn resume(&self) -> AppResult<()> {
        debug!("Resuming audio wallpaper");
        
        // TODO: Implement audio wallpaper resuming
        error!("Audio wallpaper resuming not implemented yet");
        Err(AppError::WallpaperError("Audio wallpaper resuming not implemented yet".to_string()))
    }
} 