pub mod config;
pub mod error;
pub mod plugin;
pub mod scheduler;
pub mod types;

pub use config::Config;
pub use error::AppError;
pub use plugin::{Plugin, PluginConfig, PluginManager, PluginMetadata};
pub use scheduler::{ScheduleItem, TriggerType, WallpaperScheduler};
pub use types::{WallpaperInfo, WallpaperType};

/// Application result type
pub type AppResult<T> = Result<T, AppError>; 