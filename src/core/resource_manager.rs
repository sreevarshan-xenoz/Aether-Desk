//! Resource management for wallpapers and widgets
use log::{debug, info};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Resource usage statistics
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// Memory usage in bytes
    pub memory_used: u64,
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// GPU memory usage in bytes
    pub gpu_memory_used: u64,
    /// Number of active processes
    pub active_processes: u32,
}

/// Resource limits for wallpapers
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum memory usage in bytes
    pub max_memory: u64,
    /// Maximum CPU usage percentage
    pub max_cpu: f32,
    /// Maximum GPU memory usage in bytes
    pub max_gpu_memory: u64,
    /// Maximum number of active processes
    pub max_processes: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: 1024 * 1024 * 512, // 512 MB
            max_cpu: 80.0,                 // 80%
            max_gpu_memory: 1024 * 1024 * 256, // 256 MB
            max_processes: 10,
        }
    }
}

/// Resource manager for tracking and limiting resource usage
pub struct ResourceManager {
    /// Current resource usage
    usage: Arc<RwLock<ResourceUsage>>,
    /// Resource limits
    limits: ResourceLimits,
    /// Active resource IDs
    active_resources: Arc<RwLock<HashMap<String, ResourceUsage>>>,
    /// Total memory allocated counter
    total_allocated: AtomicU64,
    /// Total memory freed counter
    total_freed: AtomicU64,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new(limits: ResourceLimits) -> Self {
        Self {
            usage: Arc::new(RwLock::new(ResourceUsage {
                memory_used: 0,
                cpu_usage: 0.0,
                gpu_memory_used: 0,
                active_processes: 0,
            })),
            limits,
            active_resources: Arc::new(RwLock::new(HashMap::new())),
            total_allocated: AtomicU64::new(0),
            total_freed: AtomicU64::new(0),
        }
    }

    /// Register a new resource with the manager
    pub async fn register_resource(&self, id: String, usage: ResourceUsage) -> Result<(), String> {
        {
            let active = self.active_resources.read().await;

            // Check if we're already at the process limit
            if active.len() >= self.limits.max_processes as usize {
                return Err("Maximum number of active processes reached".to_string());
            }

            // Check if adding this resource would exceed limits
            let current = self.usage.read().await;
            if current.memory_used + usage.memory_used > self.limits.max_memory {
                return Err("Would exceed maximum memory limit".to_string());
            }
            if current.gpu_memory_used + usage.gpu_memory_used > self.limits.max_gpu_memory {
                return Err("Would exceed maximum GPU memory limit".to_string());
            }
        }

        // Add the resource
        {
            let mut active = self.active_resources.write().await;
            active.insert(id.clone(), usage.clone());
        }

        // Update global usage
        {
            let mut current = self.usage.write().await;
            current.memory_used += usage.memory_used;
            current.cpu_usage += usage.cpu_usage;
            current.gpu_memory_used += usage.gpu_memory_used;
            current.active_processes += 1;
        }

        self.total_allocated.fetch_add(usage.memory_used, Ordering::SeqCst);

        debug!(
            "Registered resource {}: {}MB memory, {}MB GPU memory",
            id,
            usage.memory_used / (1024 * 1024),
            usage.gpu_memory_used / (1024 * 1024)
        );

        Ok(())
    }

    /// Update resource usage
    pub async fn update_resource(&self, id: &str, new_usage: ResourceUsage) -> Result<(), String> {
        let mut active = self.active_resources.write().await;
        if let Some(old_usage) = active.get(id) {
            // Calculate difference
            let mem_diff = new_usage.memory_used as i64 - old_usage.memory_used as i64;
            let gpu_mem_diff = new_usage.gpu_memory_used as i64 - old_usage.gpu_memory_used as i64;
            let cpu_diff = new_usage.cpu_usage - old_usage.cpu_usage;

            // Update the resource
            active.insert(id.to_string(), new_usage);

            // Update global usage
            let mut current = self.usage.write().await;
            if mem_diff > 0 {
                let new_memory = (current.memory_used as i64 + mem_diff) as u64;
                if new_memory > self.limits.max_memory {
                    return Err("Would exceed maximum memory limit".to_string());
                }
                current.memory_used = new_memory;
            } else {
                // Subtraction case - use saturating arithmetic
                let abs_diff = mem_diff.abs() as u64;
                current.memory_used = current.memory_used.saturating_sub(abs_diff);
            }

            if gpu_mem_diff > 0 {
                let new_gpu_memory = (current.gpu_memory_used as i64 + gpu_mem_diff) as u64;
                if new_gpu_memory > self.limits.max_gpu_memory {
                    return Err("Would exceed maximum GPU memory limit".to_string());
                }
                current.gpu_memory_used = new_gpu_memory;
            } else {
                // Subtraction case - use saturating arithmetic
                let abs_diff = gpu_mem_diff.abs() as u64;
                current.gpu_memory_used = current.gpu_memory_used.saturating_sub(abs_diff);
            }

            current.cpu_usage = (current.cpu_usage + cpu_diff).max(0.0);

            Ok(())
        } else {
            Err("Resource not found".to_string())
        }
    }

    /// Unregister a resource from the manager
    pub async fn unregister_resource(&self, id: &str) -> Result<(), String> {
        let mut active = self.active_resources.write().await;
        if let Some(usage) = active.remove(id) {
            // Update global usage
            let mut current = self.usage.write().await;
            current.memory_used = current.memory_used.saturating_sub(usage.memory_used);
            current.gpu_memory_used = current.gpu_memory_used.saturating_sub(usage.gpu_memory_used);
            current.cpu_usage = (current.cpu_usage - usage.cpu_usage).max(0.0);
            current.active_processes = current.active_processes.saturating_sub(1);

            self.total_freed.fetch_add(usage.memory_used, Ordering::SeqCst);

            debug!(
                "Unregistered resource {}: {}MB memory freed",
                id,
                usage.memory_used / (1024 * 1024)
            );

            Ok(())
        } else {
            Err("Resource not found".to_string())
        }
    }

    /// Get current resource usage
    pub async fn get_usage(&self) -> ResourceUsage {
        self.usage.read().await.clone()
    }

    /// Get resource usage for a specific resource
    pub async fn get_resource_usage(&self, id: &str) -> Option<ResourceUsage> {
        let active = self.active_resources.read().await;
        active.get(id).cloned()
    }

    /// Check if resource usage is within limits
    pub async fn is_within_limits(&self) -> bool {
        let usage = self.usage.read().await;
        usage.memory_used <= self.limits.max_memory
            && usage.gpu_memory_used <= self.limits.max_gpu_memory
            && usage.cpu_usage <= self.limits.max_cpu
            && usage.active_processes <= self.limits.max_processes
    }

    /// Get resource utilization percentage
    pub async fn get_utilization(&self) -> (f32, f32, f32) {
        let usage = self.usage.read().await;
        let memory_util = (usage.memory_used as f32 / self.limits.max_memory as f32).min(1.0) * 100.0;
        let gpu_util = (usage.gpu_memory_used as f32 / self.limits.max_gpu_memory as f32).min(1.0) * 100.0;
        let cpu_util = (usage.cpu_usage / self.limits.max_cpu).min(1.0) * 100.0;
        
        (memory_util, gpu_util, cpu_util)
    }

    /// Get total allocated and freed memory
    pub fn get_allocation_stats(&self) -> (u64, u64) {
        (
            self.total_allocated.load(Ordering::SeqCst),
            self.total_freed.load(Ordering::SeqCst),
        )
    }

    /// Perform garbage collection to clean up unused resources
    pub async fn garbage_collect(&self) -> usize {
        let active = self.active_resources.read().await;
        let initial_count = active.len();

        // In a real implementation, we would check if resources are still alive
        // For now, we'll just log the activity
        info!("Resource manager garbage collection completed. Active resources: {}", initial_count);

        0 // No resources were collected in this basic implementation
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new(ResourceLimits::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_resource_registration() {
        let rm = ResourceManager::new(ResourceLimits::default());
        
        let usage = ResourceUsage {
            memory_used: 1024 * 1024, // 1MB
            cpu_usage: 10.0,
            gpu_memory_used: 512 * 1024, // 512KB
            active_processes: 1,
        };
        
        assert!(rm.register_resource("test_resource".to_string(), usage).await.is_ok());
        
        let current_usage = rm.get_usage().await;
        assert_eq!(current_usage.memory_used, 1024 * 1024);
        assert_eq!(current_usage.cpu_usage, 10.0);
        assert_eq!(current_usage.gpu_memory_used, 512 * 1024);
        assert_eq!(current_usage.active_processes, 1);
        
        assert!(rm.unregister_resource("test_resource").await.is_ok());
        
        let final_usage = rm.get_usage().await;
        assert_eq!(final_usage.memory_used, 0);
        assert_eq!(final_usage.cpu_usage, 0.0);
        assert_eq!(final_usage.gpu_memory_used, 0);
        assert_eq!(final_usage.active_processes, 0);
    }

    #[tokio::test]
    async fn test_resource_limits() {
        let limits = ResourceLimits {
            max_memory: 1024 * 1024, // 1MB
            max_cpu: 50.0,
            max_gpu_memory: 512 * 1024, // 512KB
            max_processes: 2,
        };
        
        let rm = ResourceManager::new(limits);
        
        let usage1 = ResourceUsage {
            memory_used: 512 * 1024, // 512KB
            cpu_usage: 20.0,
            gpu_memory_used: 256 * 1024, // 256KB
            active_processes: 1,
        };
        
        let usage2 = ResourceUsage {
            memory_used: 768 * 1024, // 768KB
            cpu_usage: 40.0,
            gpu_memory_used: 384 * 1024, // 384KB
            active_processes: 1,
        };
        
        assert!(rm.register_resource("resource1".to_string(), usage1).await.is_ok());
        // This should fail due to exceeding memory limit
        assert!(rm.register_resource("resource2".to_string(), usage2).await.is_err());
        
        // Unregister first resource
        assert!(rm.unregister_resource("resource1").await.is_ok());
        
        // Now registering the second should work
        assert!(rm.register_resource("resource2".to_string(), usage2).await.is_ok());
    }

    #[tokio::test]
    async fn test_resource_updates() {
        let rm = ResourceManager::new(ResourceLimits::default());
        
        let initial_usage = ResourceUsage {
            memory_used: 1024 * 1024, // 1MB
            cpu_usage: 10.0,
            gpu_memory_used: 512 * 1024, // 512KB
            active_processes: 1,
        };
        
        assert!(rm.register_resource("test_resource".to_string(), initial_usage).await.is_ok());
        
        let updated_usage = ResourceUsage {
            memory_used: 2 * 1024 * 1024, // 2MB
            cpu_usage: 20.0,
            gpu_memory_used: 1024 * 1024, // 1MB
            active_processes: 1,
        };
        
        assert!(rm.update_resource("test_resource", updated_usage).await.is_ok());
        
        let current_usage = rm.get_usage().await;
        assert_eq!(current_usage.memory_used, 2 * 1024 * 1024);
        assert_eq!(current_usage.cpu_usage, 20.0);
        assert_eq!(current_usage.gpu_memory_used, 1024 * 1024);
    }
}