use thiserror::Error;

/// Application error
#[derive(Error, Debug)]
pub enum AppError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// Platform error
    #[error("Platform error: {0}")]
    PlatformError(String),
    
    /// Wallpaper error
    #[error("Wallpaper error: {0}")]
    WallpaperError(String),
    
    /// Plugin error
    #[error("Plugin error: {0}")]
    PluginError(String),
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// Unsupported platform
    #[error("Unsupported platform")]
    UnsupportedPlatform,
    
    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Other(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Other(s.to_string())
    }
}

/// Result type for the application
pub type AppResult<T> = Result<T, AppError>;
