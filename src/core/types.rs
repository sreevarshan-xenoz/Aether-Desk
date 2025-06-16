use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Wallpaper metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperMetadata {
    /// Wallpaper name
    pub name: String,
    
    /// Wallpaper description
    pub description: Option<String>,
    
    /// Wallpaper author
    pub author: Option<String>,
    
    /// Wallpaper tags
    pub tags: Vec<String>,
    
    /// Wallpaper path
    pub path: PathBuf,
    
    /// Wallpaper type
    pub wallpaper_type: crate::core::config::WallpaperType,
}

/// Wallpaper collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperCollection {
    /// Collection name
    pub name: String,
    
    /// Collection description
    pub description: Option<String>,
    
    /// Collection wallpapers
    pub wallpapers: Vec<WallpaperMetadata>,
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin description
    pub description: Option<String>,
    
    /// Plugin author
    pub author: Option<String>,
    
    /// Plugin entry point
    pub entry_point: String,
    
    /// Plugin dependencies
    pub dependencies: Vec<String>,
} 