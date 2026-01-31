pub mod config;
pub mod error;
pub mod performance;
pub mod plugin;
pub mod resource_manager;
pub mod scheduler;
pub mod types;
pub mod widget;

pub use config::{Config, WallpaperType, Theme};
pub use error::AppError;
pub use plugin::{PluginManager};
pub use resource_manager::{ResourceManager, ResourceLimits, ResourceUsage};
pub use scheduler::{ScheduleItem, TriggerType, WallpaperScheduler};
pub use types::WallpaperInfo;
pub use widget::{WidgetConfig, WidgetManager, WidgetPosition, WidgetSize, WidgetType};

/// Application result type
pub type AppResult<T> = Result<T, AppError>; 