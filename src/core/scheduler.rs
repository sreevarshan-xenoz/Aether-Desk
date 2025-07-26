use crate::core::{AppError, AppResult, Config, WallpaperInfo, WallpaperType};
use crate::platform::WallpaperManager;
use crate::wallpapers::{AudioWallpaper, ShaderWallpaper, StaticWallpaper, VideoWallpaper, WebWallpaper, Wallpaper};
use chrono::{DateTime, Duration, Local, NaiveTime, Timelike};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration as StdDuration;

/// Schedule trigger type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TriggerType {
    /// Time-based trigger (hour:minute)
    Time(NaiveTime),
    
    /// Interval-based trigger (hours, minutes, seconds)
    Interval(Duration),
    
    /// System event trigger (startup, shutdown, etc.)
    SystemEvent(String),
    
    /// Custom trigger (user-defined)
    Custom(String),
}

/// Schedule item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleItem {
    /// Trigger type
    pub trigger: TriggerType,
    
    /// Wallpaper information
    pub wallpaper: WallpaperInfo,
    
    /// Whether the schedule item is enabled
    pub enabled: bool,
}

/// Wallpaper scheduler
pub struct WallpaperScheduler {
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
    
    /// Schedule items
    schedule_items: Arc<Mutex<Vec<ScheduleItem>>>,
    
    /// Current wallpaper
    current_wallpaper: Arc<Mutex<Option<Box<dyn Wallpaper + Send + Sync>>>>,
    
    /// Scheduler thread handle
    scheduler_thread: Option<thread::JoinHandle<()>>,
    
    /// Whether the scheduler is running
    is_running: Arc<Mutex<bool>>,
    
    /// Last check time
    last_check: Arc<Mutex<DateTime<Local>>>,
}

impl WallpaperScheduler {
    /// Create a new wallpaper scheduler
    pub fn new(wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            wallpaper_manager,
            schedule_items: Arc::new(Mutex::new(Vec::new())),
            current_wallpaper: Arc::new(Mutex::new(None)),
            scheduler_thread: None,
            is_running: Arc::new(Mutex::new(false)),
            last_check: Arc::new(Mutex::new(Local::now())),
        }
    }
    
    /// Load schedule items from configuration
    pub fn load_schedule(&mut self, config: &Config) -> AppResult<()> {
        let schedule_file = config.get_schedule_file();
        
        if !schedule_file.exists() {
            debug!("Schedule file does not exist, creating default schedule");
            self.create_default_schedule(&schedule_file)?;
            return Ok(());
        }
        
        let schedule_content = std::fs::read_to_string(&schedule_file)
            .map_err(|e| AppError::ConfigError(format!("Failed to read schedule file: {}", e)))?;
        
        let schedule_items: Vec<ScheduleItem> = serde_json::from_str(&schedule_content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse schedule file: {}", e)))?;
        
        let mut items = self.schedule_items.lock().unwrap();
        *items = schedule_items;
        
        info!("Loaded {} schedule items", items.len());
        Ok(())
    }
    
    /// Save schedule items to configuration
    pub fn save_schedule(&self, config: &Config) -> AppResult<()> {
        let schedule_file = config.get_schedule_file();
        let items = self.schedule_items.lock().unwrap();
        
        let schedule_content = serde_json::to_string_pretty(&*items)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize schedule: {}", e)))?;
        
        std::fs::write(&schedule_file, schedule_content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write schedule file: {}", e)))?;
        
        info!("Saved {} schedule items", items.len());
        Ok(())
    }
    
    /// Create default schedule
    fn create_default_schedule(&self, schedule_file: &Path) -> AppResult<()> {
        let default_items = vec![
            ScheduleItem {
                trigger: TriggerType::Time(NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
                wallpaper: WallpaperInfo {
                    name: "Morning".to_string(),
                    description: "Morning wallpaper".to_string(),
                    author: "Aether-Desk".to_string(),
                    version: "1.0.0".to_string(),
                    r#type: WallpaperType::Static,
                    path: Some(PathBuf::from("assets/wallpapers/morning.jpg")),
                    url: None,
                },
                enabled: true,
            },
            ScheduleItem {
                trigger: TriggerType::Time(NaiveTime::from_hms_opt(18, 0, 0).unwrap()),
                wallpaper: WallpaperInfo {
                    name: "Evening".to_string(),
                    description: "Evening wallpaper".to_string(),
                    author: "Aether-Desk".to_string(),
                    version: "1.0.0".to_string(),
                    r#type: WallpaperType::Static,
                    path: Some(PathBuf::from("assets/wallpapers/evening.jpg")),
                    url: None,
                },
                enabled: true,
            },
        ];
        
        let schedule_content = serde_json::to_string_pretty(&default_items)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize default schedule: {}", e)))?;
        
        std::fs::write(schedule_file, schedule_content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write default schedule file: {}", e)))?;
        
        let mut items = self.schedule_items.lock().unwrap();
        *items = default_items;
        
        info!("Created default schedule with {} items", items.len());
        Ok(())
    }
    
    /// Start the scheduler
    pub fn start(&mut self) -> AppResult<()> {
        let is_running = *self.is_running.lock().unwrap();
        if is_running {
            debug!("Scheduler is already running");
            return Ok(());
        }
        
        *self.is_running.lock().unwrap() = true;
        
        let wallpaper_manager = self.wallpaper_manager.clone();
        let schedule_items = self.schedule_items.clone();
        let current_wallpaper = self.current_wallpaper.clone();
        let is_running = self.is_running.clone();
        let last_check = self.last_check.clone();
        
        self.scheduler_thread = Some(thread::spawn(move || {
            let check_interval = StdDuration::from_secs(60); // Check every minute
            
            while *is_running.lock().unwrap() {
                let now = Local::now();
                let mut last_check_time = last_check.lock().unwrap();
                
                // Check if a minute has passed
                if now.signed_duration_since(*last_check_time) >= chrono::Duration::minutes(1) {
                    *last_check_time = now;
                    
                    // Check schedule items
                    let items = schedule_items.lock().unwrap();
                    for item in items.iter() {
                        if !item.enabled {
                            continue;
                        }
                        
                        match &item.trigger {
                            TriggerType::Time(time) => {
                                // Check if current time matches the trigger time
                                let current_time = now.time();
                                if current_time.hour() == time.hour() && current_time.minute() == time.minute() {
                                    debug!("Time trigger activated: {:?}", time);
                                    Self::apply_wallpaper(&wallpaper_manager, &current_wallpaper, &item.wallpaper);
                                }
                            },
                            TriggerType::Interval(interval) => {
                                // Check if the interval has passed
                                // This is a simplified implementation
                                // A more robust implementation would track the last time each interval was triggered
                                debug!("Interval trigger activated: {:?}", interval);
                                Self::apply_wallpaper(&wallpaper_manager, &current_wallpaper, &item.wallpaper);
                            },
                            TriggerType::SystemEvent(event) => {
                                // System events are not implemented in this version
                                debug!("System event trigger not implemented: {}", event);
                            },
                            TriggerType::Custom(trigger) => {
                                // Custom triggers are not implemented in this version
                                debug!("Custom trigger not implemented: {}", trigger);
                            },
                        }
                    }
                }
                
                thread::sleep(check_interval);
            }
        }));
        
        info!("Scheduler started");
        Ok(())
    }
    
    /// Stop the scheduler
    pub fn stop(&mut self) -> AppResult<()> {
        let is_running = *self.is_running.lock().unwrap();
        if !is_running {
            debug!("Scheduler is not running");
            return Ok(());
        }
        
        *self.is_running.lock().unwrap() = false;
        
        if let Some(thread) = self.scheduler_thread.take() {
            thread.join().map_err(|e| {
                AppError::Other(format!("Failed to join scheduler thread: {:?}", e))
            })?;
        }
        
        info!("Scheduler stopped");
        Ok(())
    }
    
    /// Add a schedule item
    pub fn add_schedule_item(&self, item: ScheduleItem) -> AppResult<()> {
        let mut items = self.schedule_items.lock().unwrap();
        items.push(item);
        info!("Added schedule item");
        Ok(())
    }
    
    /// Remove a schedule item
    pub fn remove_schedule_item(&self, index: usize) -> AppResult<()> {
        let mut items = self.schedule_items.lock().unwrap();
        if index < items.len() {
            items.remove(index);
            info!("Removed schedule item at index {}", index);
        } else {
            return Err(AppError::Other(format!("Invalid schedule item index: {}", index)));
        }
        Ok(())
    }
    
    /// Update a schedule item
    pub fn update_schedule_item(&self, index: usize, item: ScheduleItem) -> AppResult<()> {
        let mut items = self.schedule_items.lock().unwrap();
        if index < items.len() {
            items[index] = item;
            info!("Updated schedule item at index {}", index);
        } else {
            return Err(AppError::Other(format!("Invalid schedule item index: {}", index)));
        }
        Ok(())
    }
    
    /// Get all schedule items
    pub fn get_schedule_items(&self) -> Vec<ScheduleItem> {
        let items = self.schedule_items.lock().unwrap();
        items.clone()
    }
    
    /// Apply a wallpaper
    fn apply_wallpaper(
        wallpaper_manager: &Arc<dyn WallpaperManager + Send + Sync>,
        current_wallpaper: &Arc<Mutex<Option<Box<dyn Wallpaper + Send + Sync>>>>,
        wallpaper_info: &WallpaperInfo,
    ) {
        // Stop current wallpaper if any
        if let Some(wallpaper) = &mut *current_wallpaper.lock().unwrap() {
            let rt = tokio::runtime::Runtime::new().unwrap();
            if let Err(e) = rt.block_on(wallpaper.stop()) {
                error!("Failed to stop current wallpaper: {}", e);
            }
        }
        
        // Create and start new wallpaper
        let wallpaper: Box<dyn Wallpaper + Send + Sync> = match wallpaper_info.r#type {
            WallpaperType::Static => {
                if let Some(path) = &wallpaper_info.path {
                    Box::new(StaticWallpaper::new(path, wallpaper_manager.clone()))
                } else {
                    error!("Static wallpaper path is missing");
                    return;
                }
            },
            WallpaperType::Video => {
                if let Some(path) = &wallpaper_info.path {
                    Box::new(VideoWallpaper::new(path, wallpaper_manager.clone()))
                } else {
                    error!("Video wallpaper path is missing");
                    return;
                }
            },
            WallpaperType::Web => {
                if let Some(url) = &wallpaper_info.url {
                    Box::new(WebWallpaper::new(url, wallpaper_manager.clone()))
                } else {
                    error!("Web wallpaper URL is missing");
                    return;
                }
            },
            WallpaperType::Shader => {
                if let Some(path) = &wallpaper_info.path {
                    Box::new(ShaderWallpaper::new(path, wallpaper_manager.clone()))
                } else {
                    error!("Shader wallpaper path is missing");
                    return;
                }
            },
            WallpaperType::Audio => {
                if let Some(path) = &wallpaper_info.path {
                    Box::new(AudioWallpaper::new(path, wallpaper_manager.clone()))
                } else {
                    error!("Audio wallpaper path is missing");
                    return;
                }
            },
        };
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        if let Err(e) = rt.block_on(wallpaper.start()) {
            error!("Failed to start wallpaper: {}", e);
            return;
        }
        
        *current_wallpaper.lock().unwrap() = Some(wallpaper);
        info!("Applied wallpaper: {}", wallpaper_info.name);
    }
} 