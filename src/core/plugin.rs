use crate::core::{AppResult, Config, WallpaperType};
use crate::platform::WallpaperManager;
use crate::wallpapers::Wallpaper;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin author
    pub author: String,
    
    /// Plugin description
    pub description: String,
    
    /// Plugin homepage
    pub homepage: Option<String>,
    
    /// Plugin license
    pub license: Option<String>,
    
    /// Plugin dependencies
    pub dependencies: Vec<String>,
    
    /// Plugin wallpaper types
    pub wallpaper_types: Vec<WallpaperType>,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Plugin enabled
    pub enabled: bool,
    
    /// Plugin settings
    pub settings: HashMap<String, serde_json::Value>,
}

/// Plugin trait
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;
    
    /// Initialize plugin
    fn init(&self, config: &Config) -> AppResult<()>;
    
    /// Create wallpaper
    fn create_wallpaper(&self, wallpaper_type: WallpaperType, path: &Path, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> AppResult<Box<dyn Wallpaper + Send + Sync>>;
    
    /// Get plugin settings
    fn get_settings(&self) -> &PluginConfig;
    
    /// Update plugin settings
    fn update_settings(&mut self, settings: HashMap<String, serde_json::Value>) -> AppResult<()>;
}

/// Plugin manager
pub struct PluginManager {
    /// Plugin directory
    plugin_dir: PathBuf,
    
    /// Loaded plugins
    plugins: HashMap<String, Box<dyn Plugin>>,
    
    /// Plugin configurations
    plugin_configs: HashMap<String, PluginConfig>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new(plugin_dir: &Path) -> Self {
        Self {
            plugin_dir: plugin_dir.to_path_buf(),
            plugins: HashMap::new(),
            plugin_configs: HashMap::new(),
        }
    }
    
    /// Load plugins
    pub fn load_plugins(&mut self, config: &Config) -> AppResult<()> {
        info!("Loading plugins from {}", self.plugin_dir.display());
        
        // Create plugin directory if it doesn't exist
        if !self.plugin_dir.exists() {
            std::fs::create_dir_all(&self.plugin_dir)?;
            info!("Created plugin directory: {}", self.plugin_dir.display());
        }
        
        // Load plugin configurations
        self.load_plugin_configs()?;
        
        // Load plugins
        for entry in std::fs::read_dir(&self.plugin_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.load_plugin(&path, config)?;
            }
        }
        
        info!("Loaded {} plugins", self.plugins.len());
        Ok(())
    }
    
    /// Load a plugin
    fn load_plugin(&mut self, plugin_dir: &Path, config: &Config) -> AppResult<()> {
        let plugin_name = plugin_dir.file_name().unwrap().to_string_lossy().to_string();
        info!("Loading plugin: {}", plugin_name);
        
        // Check if plugin is enabled
        let plugin_config = self.plugin_configs.get(&plugin_name).cloned().unwrap_or_else(|| {
            info!("No configuration found for plugin: {}", plugin_name);
            PluginConfig {
                enabled: true,
                settings: HashMap::new(),
            }
        });
        
        if !plugin_config.enabled {
            info!("Plugin {} is disabled, skipping", plugin_name);
            return Ok(());
        }
        
        // TODO: Implement dynamic plugin loading
        // For now, we'll just log that we would load the plugin
        info!("Would load plugin: {}", plugin_name);
        
        Ok(())
    }
    
    /// Load plugin configurations
    fn load_plugin_configs(&mut self) -> AppResult<()> {
        let config_path = self.plugin_dir.join("plugins.json");
        
        if config_path.exists() {
            let config_str = std::fs::read_to_string(&config_path)?;
            self.plugin_configs = serde_json::from_str(&config_str)?;
            info!("Loaded plugin configurations from {}", config_path.display());
        } else {
            info!("No plugin configurations found at {}", config_path.display());
        }
        
        Ok(())
    }
    
    /// Save plugin configurations
    pub fn save_plugin_configs(&self) -> AppResult<()> {
        let config_path = self.plugin_dir.join("plugins.json");
        let config_str = serde_json::to_string_pretty(&self.plugin_configs)?;
        std::fs::write(&config_path, config_str)?;
        info!("Saved plugin configurations to {}", config_path.display());
        Ok(())
    }
    
    /// Get plugin
    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(name)
    }
    
    /// Get all plugins
    pub fn get_plugins(&self) -> &HashMap<String, Box<dyn Plugin>> {
        &self.plugins
    }
    
    /// Enable plugin
    pub fn enable_plugin(&mut self, name: &str) -> AppResult<()> {
        if let Some(config) = self.plugin_configs.get_mut(name) {
            config.enabled = true;
            info!("Enabled plugin: {}", name);
        } else {
            return Err(crate::core::AppError::PluginError(format!("Plugin not found: {}", name)).into());
        }
        
        self.save_plugin_configs()?;
        Ok(())
    }
    
    /// Disable plugin
    pub fn disable_plugin(&mut self, name: &str) -> AppResult<()> {
        if let Some(config) = self.plugin_configs.get_mut(name) {
            config.enabled = false;
            info!("Disabled plugin: {}", name);
        } else {
            return Err(crate::core::AppError::PluginError(format!("Plugin not found: {}", name)).into());
        }
        
        self.save_plugin_configs()?;
        Ok(())
    }
    
    /// Update plugin settings
    pub fn update_plugin_settings(&mut self, name: &str, settings: HashMap<String, serde_json::Value>) -> AppResult<()> {
        if let Some(config) = self.plugin_configs.get_mut(name) {
            config.settings = settings;
            info!("Updated settings for plugin: {}", name);
        } else {
            return Err(crate::core::AppError::PluginError(format!("Plugin not found: {}", name)).into());
        }
        
        if let Some(plugin) = self.plugins.get_mut(name) {
            plugin.update_settings(settings)?;
        }
        
        self.save_plugin_configs()?;
        Ok(())
    }
} 