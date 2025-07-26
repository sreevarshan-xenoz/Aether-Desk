use log::{debug, warn};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Performance metrics for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Memory usage in MB
    pub memory_usage: f64,
    /// Frame time in milliseconds
    pub frame_time: f32,
    /// FPS
    pub fps: f32,
    /// Wallpaper load time in milliseconds
    pub wallpaper_load_time: u128,
}

/// Performance monitor to track application performance
#[allow(dead_code)]
pub struct PerformanceMonitor {
    /// Start time for measuring operations
    operation_start_times: HashMap<String, Instant>,
    /// Performance history
    metrics_history: Vec<PerformanceMetrics>,
    /// Maximum history size
    max_history_size: usize,
    /// Last frame time
    last_frame_time: Instant,
    /// Frame count for FPS calculation
    frame_count: u32,
    /// Time for FPS calculation
    fps_timer: Instant,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            operation_start_times: HashMap::new(),
            metrics_history: Vec::new(),
            max_history_size: 100,
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps_timer: Instant::now(),
        }
    }

    /// Start timing an operation
    pub fn start_timing(&mut self, operation: &str) {
        self.operation_start_times.insert(operation.to_string(), Instant::now());
        debug!("Started timing operation: {}", operation);
    }

    /// End timing an operation and return the duration
    pub fn end_timing(&mut self, operation: &str) -> Option<Duration> {
        if let Some(start_time) = self.operation_start_times.remove(operation) {
            let duration = start_time.elapsed();
            debug!("Operation '{}' took: {:?}", operation, duration);
            Some(duration)
        } else {
            warn!("No start time found for operation: {}", operation);
            None
        }
    }

    /// Update frame timing
    pub fn update_frame_timing(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time).as_millis() as f32;
        self.last_frame_time = now;
        
        self.frame_count += 1;
        
        // Calculate FPS every second
        if self.fps_timer.elapsed().as_secs() >= 1 {
            let fps = self.frame_count as f32 / self.fps_timer.elapsed().as_secs_f32();
            
            // Update metrics
            self.update_metrics(PerformanceMetrics {
                cpu_usage: self.get_cpu_usage(),
                memory_usage: self.get_memory_usage(),
                frame_time,
                fps,
                wallpaper_load_time: 0, // Will be updated separately
            });
            
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }
    }

    /// Update performance metrics
    pub fn update_metrics(&mut self, metrics: PerformanceMetrics) {
        self.metrics_history.push(metrics);
        
        // Keep only the last N metrics
        if self.metrics_history.len() > self.max_history_size {
            self.metrics_history.remove(0);
        }
    }

    /// Get current performance metrics
    pub fn get_current_metrics(&self) -> Option<&PerformanceMetrics> {
        self.metrics_history.last()
    }

    /// Get performance metrics history
    pub fn get_metrics_history(&self) -> &[PerformanceMetrics] {
        &self.metrics_history
    }

    /// Get average FPS over the last N frames
    pub fn get_average_fps(&self, frames: usize) -> f32 {
        let frames = frames.min(self.metrics_history.len());
        if frames == 0 {
            return 0.0;
        }

        let total_fps: f32 = self.metrics_history
            .iter()
            .rev()
            .take(frames)
            .map(|m| m.fps)
            .sum();

        total_fps / frames as f32
    }

    /// Check if performance is below acceptable thresholds
    pub fn is_performance_degraded(&self) -> bool {
        if let Some(metrics) = self.get_current_metrics() {
            metrics.fps < 30.0 || metrics.memory_usage > 500.0 || metrics.cpu_usage > 80.0
        } else {
            false
        }
    }

    /// Get CPU usage (simplified implementation)
    fn get_cpu_usage(&self) -> f32 {
        // This is a placeholder - you'd want to use a proper system monitoring library
        // like `sysinfo` for accurate CPU usage
        0.0
    }

    /// Get memory usage in MB
    fn get_memory_usage(&self) -> f64 {
        // This is a placeholder - you'd want to use a proper system monitoring library
        // In a real implementation, you might use `sysinfo` or similar
        0.0
    }
}

/// Macro for easy performance timing
#[macro_export]
macro_rules! time_operation {
    ($monitor:expr, $operation:expr, $code:block) => {{
        $monitor.start_timing($operation);
        let result = $code;
        $monitor.end_timing($operation);
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_performance_monitor_timing() {
        let mut monitor = PerformanceMonitor::new();
        
        monitor.start_timing("test_operation");
        thread::sleep(Duration::from_millis(10));
        let duration = monitor.end_timing("test_operation");
        
        assert!(duration.is_some());
        assert!(duration.unwrap().as_millis() >= 10);
    }

    #[test]
    fn test_metrics_history_limit() {
        let mut monitor = PerformanceMonitor::new();
        monitor.max_history_size = 5;
        
        for i in 0..10 {
            monitor.update_metrics(PerformanceMetrics {
                cpu_usage: i as f32,
                memory_usage: i as f64,
                frame_time: i as f32,
                fps: i as f32,
                wallpaper_load_time: i as u128,
            });
        }
        
        assert_eq!(monitor.metrics_history.len(), 5);
        assert_eq!(monitor.metrics_history[0].cpu_usage, 5.0);
    }

    #[test]
    fn test_average_fps_calculation() {
        let mut monitor = PerformanceMonitor::new();
        
        for i in 1..=5 {
            monitor.update_metrics(PerformanceMetrics {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                frame_time: 0.0,
                fps: i as f32 * 10.0,
                wallpaper_load_time: 0,
            });
        }
        
        let avg_fps = monitor.get_average_fps(3);
        assert_eq!(avg_fps, 40.0); // (30 + 40 + 50) / 3
    }
}
