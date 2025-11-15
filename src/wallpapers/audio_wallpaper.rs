use crate::core::{AppError, AppResult, WallpaperType};
use crate::platform::WallpaperManager;
use log::{debug, error, info};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;

/// Audio wallpaper
pub struct AudioWallpaper {
    /// Audio path
    path: PathBuf,
    
    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,
    
    /// Whether the audio wallpaper is active
    is_active: Arc<Mutex<bool>>,

    /// Audio visualizer process ID for control
    audio_pid: Arc<Mutex<Option<u32>>>,
}

impl AudioWallpaper {
    /// Create a new audio wallpaper
    pub fn new<P: AsRef<Path>>(path: P, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            wallpaper_manager,
            is_active: Arc::new(Mutex::new(false)),
            audio_pid: Arc::new(Mutex::new(None)),
        }
    }

    /// Find audio visualizer process ID for the current audio file
    async fn find_audio_process(&self) -> AppResult<Option<u32>> {
        let path_str = self.path.to_string_lossy().to_string();

        #[cfg(target_os = "windows")]
        {
            let output = Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!("Get-Process -Name shadertoy,vlc,audacious | Where-Object {{ $_.CommandLine -like '*{}*' }} | Select-Object -ExpandProperty Id", path_str)
                ])
                .output()?;

            if output.status.success() {
                let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !pid_str.is_empty() {
                    return Ok(Some(pid_str.parse::<u32>().map_err(|_| AppError::WallpaperError("Failed to parse audio PID".to_string()))?));
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            for audio_tool in &["shadertoy", "vlc", "audacious", "cava"] {
                let output = Command::new("pgrep")
                    .args(&["-f", &format!("{}.*{}", audio_tool, path_str)])
                    .output()?;

                if output.status.success() {
                    let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !pid_str.is_empty() {
                        return Ok(Some(pid_str.parse::<u32>().map_err(|_| AppError::WallpaperError("Failed to parse audio PID".to_string()))?));
                    }
                }
            }
        }

        Ok(None)
    }
}

#[async_trait]
impl super::Wallpaper for AudioWallpaper {
    fn get_type(&self) -> WallpaperType {
        WallpaperType::Audio
    }
    
    fn get_path(&self) -> Option<&Path> {
        Some(&self.path)
    }
    
    async fn start(&self) -> AppResult<()> {
        debug!("Starting audio wallpaper: {:?}", self.path);

        // Set the wallpaper using the platform-specific manager
        self.wallpaper_manager.set_audio_wallpaper(&self.path).await?;

        // Try to find and store the audio visualizer process ID
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await; // Give audio visualizer time to start
        if let Ok(Some(pid)) = self.find_audio_process().await {
            let mut audio_pid = self.audio_pid.lock().await;
            *audio_pid = Some(pid);
            debug!("Found audio visualizer process ID: {}", pid);
        }

        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = true;

        info!("Audio wallpaper started");
        Ok(())
    }
    
    async fn stop(&self) -> AppResult<()> {
        debug!("Stopping audio wallpaper");

        // Stop the wallpaper using the platform-specific manager
        self.wallpaper_manager.stop_wallpaper().await?;

        // Clear audio PID and active state
        let mut audio_pid = self.audio_pid.lock().await;
        *audio_pid = None;
        let mut is_active = self.is_active.lock().await;
        *is_active = false;

        info!("Audio wallpaper stopped");
        Ok(())
    }
    
    async fn pause(&self) -> AppResult<()> {
        debug!("Pausing audio wallpaper");

        let audio_pid = self.audio_pid.lock().await;

        if let Some(pid) = *audio_pid {
            #[cfg(target_os = "windows")]
            {
                // Minimize audio visualizer window on Windows
                let output = Command::new("powershell")
                    .args(&[
                        "-Command",
                        &format!("(New-Object -ComObject WScript.Shell).AppActivate('{}')", pid)
                    ])
                    .output()?;

                if output.status.success() {
                    debug!("Audio visualizer paused successfully");
                    return Ok(());
                }
            }

            #[cfg(target_os = "linux")]
            {
                // Send SIGSTOP to audio visualizer process on Linux
                let output = Command::new("kill")
                    .args(&["-STOP", &pid.to_string()])
                    .output()?;

                if output.status.success() {
                    debug!("Audio visualizer paused successfully");
                    return Ok(());
                }
            }

            error!("Failed to pause audio visualizer");
            return Err(AppError::WallpaperError("Failed to pause audio wallpaper".to_string()));
        } else {
            // Try to find audio visualizer process and pause it
            drop(audio_pid);
            if let Ok(Some(pid)) = self.find_audio_process().await {
                let mut audio_pid = self.audio_pid.lock().await;
                *audio_pid = Some(pid);
                return self.pause().await;
            }
        }

        Err(AppError::WallpaperError("Audio visualizer process not found".to_string()))
    }
    
    async fn resume(&self) -> AppResult<()> {
        debug!("Resuming audio wallpaper");

        let audio_pid = self.audio_pid.lock().await;

        if let Some(pid) = *audio_pid {
            #[cfg(target_os = "windows")]
            {
                // Restore audio visualizer window on Windows
                let output = Command::new("powershell")
                    .args(&[
                        "-Command",
                        &format!("(New-Object -ComObject WScript.Shell).AppActivate('{}')", pid)
                    ])
                    .output()?;

                if output.status.success() {
                    debug!("Audio visualizer resumed successfully");
                    return Ok(());
                }
            }

            #[cfg(target_os = "linux")]
            {
                // Send SIGCONT to audio visualizer process on Linux
                let output = Command::new("kill")
                    .args(&["-CONT", &pid.to_string()])
                    .output()?;

                if output.status.success() {
                    debug!("Audio visualizer resumed successfully");
                    return Ok(());
                }
            }

            error!("Failed to resume audio visualizer");
            return Err(AppError::WallpaperError("Failed to resume audio wallpaper".to_string()));
        } else {
            // Try to find audio visualizer process and resume it
            drop(audio_pid);
            if let Ok(Some(pid)) = self.find_audio_process().await {
                let mut audio_pid = self.audio_pid.lock().await;
                *audio_pid = Some(pid);
                return self.resume().await;
            }
        }

        Err(AppError::WallpaperError("Audio visualizer process not found".to_string()))
    }
} 