use crate::core::{AppError, AppResult, WallpaperType};
use crate::platform::WallpaperManager;
use log::{debug, error, info};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;

/// Shader wallpaper
pub struct ShaderWallpaper {
    /// Shader path
    path: PathBuf,

    /// Platform-specific wallpaper manager
    wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>,

    /// Whether the shader is active
    is_active: Arc<Mutex<bool>>,

    /// Shader process ID for control
    shader_pid: Arc<Mutex<Option<u32>>>,
}

impl ShaderWallpaper {
    /// Create a new shader wallpaper
    pub fn new<P: AsRef<Path>>(path: P, wallpaper_manager: Arc<dyn WallpaperManager + Send + Sync>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            wallpaper_manager,
            is_active: Arc::new(Mutex::new(false)),
            shader_pid: Arc::new(Mutex::new(None)),
        }
    }

    /// Find shader process ID for the current shader
    async fn find_shader_process(&self) -> AppResult<Option<u32>> {
        let path_str = self.path.to_string_lossy().to_string();

        #[cfg(target_os = "windows")]
        {
            let output = Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!("Get-Process -Name shadertoy,glslviewer | Where-Object {{ $_.CommandLine -like '*{}*' }} | Select-Object -ExpandProperty Id", path_str)
                ])
                .output()?;

            if output.status.success() {
                let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !pid_str.is_empty() {
                    return Ok(Some(pid_str.parse::<u32>().map_err(|_| AppError::WallpaperError("Failed to parse shader PID".to_string()))?));
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            for shader_tool in &["shadertoy", "glslviewer"] {
                let output = Command::new("pgrep")
                    .args(&["-f", &format!("{}.*{}", shader_tool, path_str)])
                    .output()?;

                if output.status.success() {
                    let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !pid_str.is_empty() {
                        return Ok(Some(pid_str.parse::<u32>().map_err(|_| AppError::WallpaperError("Failed to parse shader PID".to_string()))?));
                    }
                }
            }
        }

        Ok(None)
    }
}

#[async_trait]
impl super::Wallpaper for ShaderWallpaper {
    fn get_type(&self) -> WallpaperType {
        WallpaperType::Shader
    }
    
    fn get_path(&self) -> Option<&Path> {
        Some(&self.path)
    }
    
    async fn start(&self) -> AppResult<()> {
        debug!("Starting shader wallpaper: {:?}", self.path);

        // Set the wallpaper using the platform-specific manager
        self.wallpaper_manager.set_shader_wallpaper(&self.path).await?;

        // Try to find and store the shader process ID
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await; // Give shader tool time to start
        if let Ok(Some(pid)) = self.find_shader_process().await {
            let mut shader_pid = self.shader_pid.lock().await;
            *shader_pid = Some(pid);
            debug!("Found shader process ID: {}", pid);
        }

        // Update active state
        let mut is_active = self.is_active.lock().await;
        *is_active = true;

        info!("Shader wallpaper started");
        Ok(())
    }
    
    async fn stop(&self) -> AppResult<()> {
        debug!("Stopping shader wallpaper");

        // Stop the wallpaper using the platform-specific manager
        self.wallpaper_manager.stop_wallpaper().await?;

        // Clear shader PID and active state
        let mut shader_pid = self.shader_pid.lock().await;
        *shader_pid = None;
        let mut is_active = self.is_active.lock().await;
        *is_active = false;

        info!("Shader wallpaper stopped");
        Ok(())
    }
    
    async fn pause(&self) -> AppResult<()> {
        debug!("Pausing shader wallpaper");

        let shader_pid = self.shader_pid.lock().await;

        if let Some(pid) = *shader_pid {
            #[cfg(target_os = "windows")]
            {
                // Minimize shader window on Windows
                let output = Command::new("powershell")
                    .args(&[
                        "-Command",
                        &format!("(New-Object -ComObject WScript.Shell).AppActivate('{}')", pid)
                    ])
                    .output()?;

                if output.status.success() {
                    debug!("Shader paused successfully");
                    return Ok(());
                }
            }

            #[cfg(target_os = "linux")]
            {
                // Send SIGSTOP to shader process on Linux
                let output = Command::new("kill")
                    .args(&["-STOP", &pid.to_string()])
                    .output()?;

                if output.status.success() {
                    debug!("Shader paused successfully");
                    return Ok(());
                }
            }

            error!("Failed to pause shader");
            return Err(AppError::WallpaperError("Failed to pause shader".to_string()));
        } else {
            // Try to find shader process and pause it
            drop(shader_pid);
            if let Ok(Some(pid)) = self.find_shader_process().await {
                let mut shader_pid = self.shader_pid.lock().await;
                *shader_pid = Some(pid);
                return self.pause().await;
            }
        }

        Err(AppError::WallpaperError("Shader process not found".to_string()))
    }
    
    async fn resume(&self) -> AppResult<()> {
        debug!("Resuming shader wallpaper");
        
        // TODO: Implement shader resuming
        error!("Shader resuming not implemented yet");
        Err(AppError::WallpaperError("Shader resuming not implemented yet".to_string()))
    }
} 