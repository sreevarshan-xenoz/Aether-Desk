use thiserror::Error;

/// Application error type
#[derive(Error, Debug)]
pub enum AppError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// Wallpaper error
    #[error("Wallpaper error: {0}")]
    Wallpaper(String),
    
    /// Platform-specific error
    #[error("Platform error: {0}")]
    Platform(String),
    
    /// Plugin error
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for the application
pub type AppResult<T> = Result<T, AppError>; 