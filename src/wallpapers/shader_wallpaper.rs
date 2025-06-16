use crate::core::{AppError, AppResult, WallpaperType};
use crate::platform::WallpaperManager;
use log::{debug, error, info};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Shader wallpaper
pub struct ShaderWallpaper {
    /// Shader path
    path: PathBuf,
    
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
    
    /// Whether the shader is active
    is_active: Arc<Mutex<bool>>,
}

impl ShaderWallpaper {
    /// Create a new shader wallpaper
    pub fn new<P: AsRef<Path>>(path: P, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            wallpaper_manager,
            is_active: Arc::new(Mutex::new(false)),
        }
    }
}

impl super::Wallpaper for ShaderWallpaper {
    fn get_type(&self) -> WallpaperType {
        WallpaperType::Shader
    }
    
    fn get_path(&self) -> Option<&Path> {
        Some(&self.path)
    }
    
    fn start(&self) -> AppResult<()> {
        debug!("Starting shader wallpaper: {:?}", self.path);
        
        // Set the wallpaper using the platform-specific manager
        self.wallpaper_manager.set_shader_wallpaper(&self.path)?;
        
        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = true;
        
        info!("Shader wallpaper started");
        Ok(())
    }
    
    fn stop(&self) -> AppResult<()> {
        debug!("Stopping shader wallpaper");
        
        // Stop the wallpaper using the platform-specific manager
        self.wallpaper_manager.stop_wallpaper()?;
        
        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = false;
        
        info!("Shader wallpaper stopped");
        Ok(())
    }
    
    fn pause(&self) -> AppResult<()> {
        debug!("Pausing shader wallpaper");
        
        // TODO: Implement shader pausing
        error!("Shader pausing not implemented yet");
        Err(AppError::Wallpaper("Shader pausing not implemented yet".to_string()))
    }
    
    fn resume(&self) -> AppResult<()> {
        debug!("Resuming shader wallpaper");
        
        // TODO: Implement shader resuming
        error!("Shader resuming not implemented yet");
        Err(AppError::Wallpaper("Shader resuming not implemented yet".to_string()))
    }
} 