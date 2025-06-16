pub mod static_wallpaper;
pub mod video_wallpaper;
pub mod web_wallpaper;
pub mod shader_wallpaper;
pub mod audio_wallpaper;

pub use static_wallpaper::*;
pub use video_wallpaper::*;
pub use web_wallpaper::*;
pub use shader_wallpaper::*;
pub use audio_wallpaper::*;

use crate::core::{AppResult, WallpaperType};
use std::path::Path;

/// Wallpaper trait
pub trait Wallpaper {
    /// Get the wallpaper type
    fn get_type(&self) -> WallpaperType;
    
    /// Get the wallpaper path
    fn get_path(&self) -> Option<&Path>;
    
    /// Start the wallpaper
    fn start(&self) -> AppResult<()>;
    
    /// Stop the wallpaper
    fn stop(&self) -> AppResult<()>;
    
    /// Pause the wallpaper
    fn pause(&self) -> AppResult<()>;
    
    /// Resume the wallpaper
    fn resume(&self) -> AppResult<()>;
} 