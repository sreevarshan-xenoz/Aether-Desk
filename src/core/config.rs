use anyhow::Result;
use dirs::config_dir;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Current wallpaper settings
    pub wallpaper: WallpaperConfig,
    
    /// Application settings
    pub app: AppConfig,
    
    /// Plugin settings
    pub plugins: PluginConfig,
}

/// Wallpaper configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperConfig {
    /// Current wallpaper path
    pub current_path: Option<String>,
    
    /// Wallpaper type
    pub wallpaper_type: WallpaperType,
    
    /// Auto-change settings
    pub auto_change: AutoChangeConfig,
}

/// Wallpaper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WallpaperType {
    /// Static image
    Static,
    
    /// Video
    Video,
    
    /// Web-based
    Web,
    
    /// Shader-based
    Shader,
    
    /// Audio-reactive
    Audio,
}

/// Auto-change configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoChangeConfig {
    /// Whether auto-change is enabled
    pub enabled: bool,
    
    /// Change interval in minutes
    pub interval: u32,
    
    /// Folder to pick wallpapers from
    pub folder: Option<String>,
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Whether to start with system
    pub start_with_system: bool,
    
    /// Whether to show in system tray
    pub show_in_tray: bool,
    
    /// Whether to minimize to tray
    pub minimize_to_tray: bool,
    
    /// Theme configuration
    pub theme: ThemeConfig,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Theme type
    pub theme: Theme,
    /// Custom accent color (if custom theme)
    pub accent_color: Option<String>,
    /// Custom background color (if custom theme)
    pub background_color: Option<String>,
}

/// Theme type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    Custom,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            accent_color: None,
            background_color: None,
        }
    }
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Enabled plugins
    pub enabled: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wallpaper: WallpaperConfig {
                current_path: None,
                wallpaper_type: WallpaperType::Static,
                auto_change: AutoChangeConfig {
                    enabled: false,
                    interval: 30,
                    folder: None,
                },
            },
            app: AppConfig {
                start_with_system: false,
                show_in_tray: true,
                minimize_to_tray: true,
                theme: ThemeConfig::default(),
            },
            plugins: PluginConfig {
                enabled: Vec::new(),
            },
        }
    }
}

impl Config {
    /// Get the configuration directory
    pub fn get_config_dir() -> Result<PathBuf> {
        let mut config_dir = config_dir().ok_or_else(|| {
            anyhow::anyhow!("Could not find configuration directory")
        })?;
        
        config_dir.push("aether-desk");
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        
        Ok(config_dir)
    }
    
    /// Get the configuration file path
    pub fn get_config_path() -> Result<PathBuf> {
        let mut config_path = Self::get_config_dir()?;
        config_path.push("config.json");
        Ok(config_path)
    }
    
    /// Get the schedule file path
    pub fn get_schedule_file(&self) -> PathBuf {
        let mut config_dir = Self::get_config_dir().unwrap_or_else(|_| {
            let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            dir.push("config");
            dir
        });
        
        config_dir.push("schedule.json");
        config_dir
    }
    
    /// Get the widgets file path
    pub fn get_widgets_file(&self) -> PathBuf {
        let mut config_dir = Self::get_config_dir().unwrap_or_else(|_| {
            let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            dir.push("config");
            dir
        });
        
        config_dir.push("widgets.json");
        config_dir
    }
    
    /// Get the plugin directory path
    pub fn get_plugin_dir(&self) -> PathBuf {
        let mut config_dir = Self::get_config_dir().unwrap_or_else(|_| {
            let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            dir.push("config");
            dir
        });
        
        config_dir.push("plugins");
        
        // Create plugins directory if it doesn't exist
        if !config_dir.exists() {
            let _ = std::fs::create_dir_all(&config_dir);
        }
        
        config_dir
    }
    
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            info!("Configuration file not found, creating default");
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }
        
        let config_str = fs::read_to_string(config_path)?;
        let config: Self = serde_json::from_str(&config_str)?;
        
        debug!("Configuration loaded");
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(config_path, config_str)?;
        
        debug!("Configuration saved");
        Ok(())
    }
} 