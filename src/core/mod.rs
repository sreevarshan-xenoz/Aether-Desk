pub mod config;
pub mod error;
pub mod performance;
pub mod plugin;
pub mod scheduler;
pub mod types;
pub mod widget;

pub use config::{Config, WallpaperType, Theme, ThemeConfig};
pub use error::AppError;
pub use plugin::{Plugin, PluginConfig, PluginManager};
pub use scheduler::{ScheduleItem, TriggerType, WallpaperScheduler};
pub use types::{WallpaperInfo, WallpaperMetadata, WallpaperCollection, PluginMetadata};
pub use widget::{Widget, WidgetConfig, WidgetManager, WidgetPosition, WidgetSize, WidgetType};

/// Application result type
pub type AppResult<T> = Result<T, AppError>; 