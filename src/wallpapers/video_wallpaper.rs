use crate::core::{AppError, AppResult, WallpaperType};
use crate::platform::WallpaperManager;
use log::{debug, error, info};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;

/// Video wallpaper
pub struct VideoWallpaper {
    /// Video path
    path: PathBuf,

    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,

    /// Whether the video is playing
    is_playing: Arc<Mutex<bool>>,

    /// VLC process ID for control
    vlc_pid: Arc<Mutex<Option<u32>>>,
}

impl VideoWallpaper {
    /// Create a new video wallpaper
    pub fn new<P: AsRef<Path>>(path: P, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            wallpaper_manager,
            is_playing: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait]
impl super::Wallpaper for VideoWallpaper {
    fn get_type(&self) -> WallpaperType {
        WallpaperType::Video
    }
    
    fn get_path(&self) -> Option<&Path> {
        Some(&self.path)
    }
    
    async fn start(&self) -> AppResult<()> {
        debug!("Starting video wallpaper: {:?}", self.path);
        
        // Set the wallpaper using the platform-specific manager
        self.wallpaper_manager.set_video_wallpaper(&self.path).await?;
        
        // Update playing state
        let mut is_playing = self.is_playing.lock().await;
        *is_playing = true;
        
        info!("Video wallpaper started");
        Ok(())
    }
    
    async fn stop(&self) -> AppResult<()> {
        debug!("Stopping video wallpaper");
        
        // Stop the wallpaper using the platform-specific manager
        self.wallpaper_manager.stop_wallpaper().await?;
        
        // Update playing state
        let mut is_playing = self.is_playing.lock().await;
        *is_playing = false;
        
        info!("Video wallpaper stopped");
        Ok(())
    }
    
    async fn pause(&self) -> AppResult<()> {
        debug!("Pausing video wallpaper");
        
        // TODO: Implement video pausing
        error!("Video pausing not implemented yet");
        Err(AppError::WallpaperError("Video pausing not implemented yet".to_string()))
    }
    
    async fn resume(&self) -> AppResult<()> {
        debug!("Resuming video wallpaper");
        
        // TODO: Implement video resuming
        error!("Video resuming not implemented yet");
        Err(AppError::WallpaperError("Video resuming not implemented yet".to_string()))
    }
} 