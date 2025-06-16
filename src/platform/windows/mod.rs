use crate::core::{AppError, AppResult};
use log::{debug, error, info};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use winapi::um::winuser::{SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE};

/// Windows wallpaper manager
pub struct WindowsWallpaperManager {
    /// Current wallpaper path
    current_wallpaper: Arc<Mutex<Option<String>>>,
}

impl WindowsWallpaperManager {
    /// Create a new Windows wallpaper manager
    pub fn new() -> Self {
        Self {
            current_wallpaper: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Initialize the Windows wallpaper manager
    pub fn init() -> AppResult<()> {
        info!("Initializing Windows wallpaper manager");
        Ok(())
    }
}

impl super::WallpaperManager for WindowsWallpaperManager {
    fn set_static_wallpaper(&self, path: &Path) -> AppResult<()> {
        let path_str = path.to_string_lossy().to_string();
        debug!("Setting static wallpaper: {}", path_str);
        
        // Convert path to wide string
        let wide_path: Vec<u16> = path_str.encode_utf16().chain(std::iter::once(0)).collect();
        
        // Set wallpaper using Windows API
        unsafe {
            if SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                wide_path.as_ptr() as *mut _,
                SPIF_UPDATEINIFILE,
            ) == 0
            {
                return Err(AppError::Platform("Failed to set wallpaper".to_string()));
            }
        }
        
        // Update current wallpaper
        let mut current = self.current_wallpaper.lock().await;
        *current = Some(path_str);
        
        info!("Static wallpaper set successfully");
        Ok(())
    }
    
    fn set_video_wallpaper(&self, path: &Path) -> AppResult<()> {
        // TODO: Implement video wallpaper using DirectX or other methods
        error!("Video wallpapers not implemented yet");
        Err(AppError::Platform("Video wallpapers not implemented yet".to_string()))
    }
    
    fn set_web_wallpaper(&self, url: &str) -> AppResult<()> {
        // TODO: Implement web wallpaper using WebView2 or other methods
        error!("Web wallpapers not implemented yet");
        Err(AppError::Platform("Web wallpapers not implemented yet".to_string()))
    }
    
    fn set_shader_wallpaper(&self, shader_path: &Path) -> AppResult<()> {
        // TODO: Implement shader wallpaper using DirectX or OpenGL
        error!("Shader wallpapers not implemented yet");
        Err(AppError::Platform("Shader wallpapers not implemented yet".to_string()))
    }
    
    fn set_audio_wallpaper(&self, shader_path: &Path) -> AppResult<()> {
        // TODO: Implement audio-reactive wallpaper
        error!("Audio-reactive wallpapers not implemented yet");
        Err(AppError::Platform("Audio-reactive wallpapers not implemented yet".to_string()))
    }
    
    fn stop_wallpaper(&self) -> AppResult<()> {
        debug!("Stopping current wallpaper");
        
        // Clear current wallpaper
        let mut current = self.current_wallpaper.lock().await;
        *current = None;
        
        info!("Wallpaper stopped");
        Ok(())
    }
    
    fn get_current_wallpaper(&self) -> AppResult<Option<String>> {
        let current = self.current_wallpaper.lock().await;
        Ok(current.clone())
    }
} 