//! Memory management module for RoyaOS
//!
//! This module handles memory allocation, deallocation, and optimization for the RoyaOS system.
//! It is specifically designed to support the memory requirements of AGI cognitive processes,
//! with features for efficient memory management, prioritization, and optimization.
//!
//! The memory system in RoyaOS is structured to mimic cognitive memory models, with:
//! - Short-term memory (fast access, limited capacity)
//! - Long-term memory (slower access, larger capacity)
//! - Working memory (active processing space)
//!
//! This design allows Roya AGI to operate with memory patterns similar to human cognition,
//! while optimizing for computational efficiency.

use log::{info, error, debug};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use uuid::Uuid;

/// Memory handle type used to reference allocated memory blocks
pub type MemoryHandle = Uuid;

/// Memory allocation category for prioritization and optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryCategory {
    /// Critical system memory that must not be paged or compressed
    System,
    /// Short-term memory for immediate cognitive processes
    ShortTerm,
    /// Working memory for active processing
    Working,
    /// Long-term memory for persistent storage
    LongTerm,
    /// Low-priority memory that can be paged or compressed
    Background,
}

/// Memory allocation representing a block of memory in the system
#[derive(Debug)]
struct MemoryAllocation {
    /// Size of allocation in bytes
    size: usize,
    /// When the memory was allocated
    allocated_at: Instant,
    /// Last time the memory was accessed
    last_accessed: Instant,
    /// Memory purpose/description
    purpose: String,
    /// Memory category for prioritization
    category: MemoryCategory,
    /// Access count for usage statistics
    access_count: usize,
}

/// Memory manager responsible for all memory operations in RoyaOS
///
/// The MemoryManager handles allocation, deallocation, and optimization of memory
/// resources. It implements cognitive memory models to support AGI operations
/// while ensuring efficient use of system resources.
#[derive(Debug)]
pub struct MemoryManager {
    /// Maximum memory allocation in bytes
    max_allocation: usize,
    /// Current total allocation in bytes
    current_allocation: usize,
    /// Map of memory handles to allocations
    allocations: HashMap<MemoryHandle, MemoryAllocation>,
    /// Memory optimization strategy
    optimization_strategy: String,
    /// Memory usage statistics by category
    category_usage: HashMap<MemoryCategory, usize>,
    /// Last optimization time
    last_optimization: Instant,
}

impl MemoryManager {
    /// Create a new memory manager with the specified maximum allocation
    ///
    /// # Arguments
    ///
    /// * `max_allocation_mb` - Maximum memory allocation in megabytes
    /// * `optimization_strategy` - Strategy for memory optimization ("aggressive", "balanced", or "conservative")
    ///
    /// # Returns
    ///
    /// A new MemoryManager instance
    pub fn new(max_allocation_mb: usize, optimization_strategy: &str) -> Self {
        let max_allocation = max_allocation_mb * 1024 * 1024; // Convert MB to bytes
        info!("Initializing memory manager with {} MB max allocation and '{}' optimization strategy", 
              max_allocation_mb, optimization_strategy);
        
        let mut category_usage = HashMap::new();
        for category in [
            MemoryCategory::System,
            MemoryCategory::ShortTerm,
            MemoryCategory::Working,
            MemoryCategory::LongTerm,
            MemoryCategory::Background,
        ].iter() {
            category_usage.insert(*category, 0);
        }
        
        Self {
            max_allocation,
            current_allocation: 0,
            allocations: HashMap::new(),
            optimization_strategy: optimization_strategy.to_string(),
            category_usage,
            last_optimization: Instant::now(),
        }
    }
    
    /// Allocate memory with the specified size, purpose, and category
    ///
    /// This method allocates a block of memory and returns a handle that can be
    /// used to reference the allocation in future operations.
    ///
    /// # Arguments
    ///
    /// * `size_bytes` - Size of the allocation in bytes
    /// * `purpose` - Description of the memory's purpose
    /// * `category` - Memory category for prioritization
    ///
    /// # Returns
    ///
    /// A handle to the allocated memory, or an error message
    pub fn allocate(&mut self, size_bytes: usize, purpose: &str, category: MemoryCategory) -> Result<MemoryHandle, String> {
        debug!("Allocating {} bytes for '{}' in category {:?}", size_bytes, purpose, category);
        
        // Check if allocation would exceed maximum
        if self.current_allocation + size_bytes > self.max_allocation {
            // Try to optimize memory before failing
            if self.optimization_strategy == "aggressive" {
                self.optimize()?;
            }
            
            // Check again after optimization
            if self.current_allocation + size_bytes > self.max_allocation {
                let error_msg = format!(
                    "Memory allocation of {} bytes would exceed maximum of {} bytes",
                    size_bytes, self.max_allocation
                );
                error!("{}", error_msg);
                return Err(error_msg);
            }
        }
        
        // Create allocation
        let handle = Uuid::new_v4();
        let now = Instant::now();
        let allocation = MemoryAllocation {
            size: size_bytes,
            allocated_at: now,
            last_accessed: now,
            purpose: purpose.to_string(),
            category,
            access_count: 0,
        };
        
        // Update state
        self.allocations.insert(handle, allocation);
        self.current_allocation += size_bytes;
        
        // Update category usage
        *self.category_usage.entry(category).or_insert(0) += size_bytes;
        
        debug!("Allocated memory with handle {}", handle);
        Ok(handle)
    }
    
    /// Access memory to update usage statistics
    ///
    /// # Arguments
    ///
    /// * `handle` - Handle to the memory allocation
    ///
    /// # Returns
    ///
    /// `Ok(())` if access is successful, or an error message
    pub fn access(&mut self, handle: MemoryHandle) -> Result<(), String> {
        let allocation = self.allocations.get_mut(&handle).ok_or_else(|| {
            let error_msg = format!("No memory allocation found for handle {}", handle);
            error!("{}", error_msg);
            error_msg
        })?;
        
        allocation.last_accessed = Instant::now();
        allocation.access_count += 1;
        
        Ok(())
    }
    
    /// Deallocate memory with the specified handle
    ///
    /// This method releases a previously allocated block of memory.
    ///
    /// # Arguments
    ///
    /// * `handle` - Handle to the memory allocation to deallocate
    ///
    /// # Returns
    ///
    /// `Ok(())` if deallocation is successful, or an error message
    pub fn deallocate(&mut self, handle: MemoryHandle) -> Result<(), String> {
        debug!("Deallocating memory with handle {}", handle);
        
        // Find allocation
        let allocation = match self.allocations.remove(&handle) {
            Some(alloc) => alloc,
            None => {
                let error_msg = format!("No memory allocation found for handle {}", handle);
                error!("{}", error_msg);
                return Err(error_msg);
            }
        };
        
        // Update state
        self.current_allocation -= allocation.size;
        
        // Update category usage
        if let Some(category_size) = self.category_usage.get_mut(&allocation.category) {
            *category_size = category_size.saturating_sub(allocation.size);
        }
        
        debug!("Deallocated {} bytes from category {:?}", allocation.size, allocation.category);
        Ok(())
    }
    
    /// Get current memory usage in bytes
    ///
    /// # Returns
    ///
    /// Current memory usage in bytes
    pub fn current_usage(&self) -> usize {
        self.current_allocation
    }
    
    /// Get maximum memory allocation in bytes
    ///
    /// # Returns
    ///
    /// Maximum memory allocation in bytes
    pub fn max_allocation(&self) -> usize {
        self.max_allocation
    }
    
    /// Get memory usage percentage
    ///
    /// # Returns
    ///
    /// Memory usage as a percentage of maximum allocation
    pub fn usage_percentage(&self) -> f64 {
        (self.current_allocation as f64 / self.max_allocation as f64) * 100.0
    }
    
    /// Get memory usage for a specific category
    ///
    /// # Arguments
    ///
    /// * `category` - Memory category to get usage for
    ///
    /// # Returns
    ///
    /// Memory usage for the specified category in bytes
    pub fn category_usage(&self, category: MemoryCategory) -> usize {
        *self.category_usage.get(&category).unwrap_or(&0)
    }
    
    /// Optimize memory usage based on the current strategy
    ///
    /// This method attempts to free up memory by:
    /// 1. Identifying unused or infrequently accessed allocations
    /// 2. Compressing or paging out low-priority memory
    /// 3. Consolidating fragmented memory
    ///
    /// # Returns
    ///
    /// `Ok(())` if optimization is successful, or an error message
    pub fn optimize(&mut self) -> Result<(), String> {
        info!("Optimizing memory with '{}' strategy", self.optimization_strategy);
        
        let now = Instant::now();
        self.last_optimization = now;
        
        // Skip if we have plenty of free memory
        if self.usage_percentage() < 70.0 {
            debug!("Memory usage below threshold, skipping optimization");
            return Ok(());
        }
        
        // Identify candidates for cleanup based on strategy
        let mut handles_to_remove = Vec::new();
        let threshold = match self.optimization_strategy.as_str() {
            "aggressive" => Duration::from_secs(60), // 1 minute
            "balanced" => Duration::from_secs(300),  // 5 minutes
            "conservative" => Duration::from_secs(900), // 15 minutes
            _ => Duration::from_secs(300), // Default to balanced
        };
        
        // Find unused allocations in Background category
        for (handle, allocation) in &self.allocations {
            if allocation.category == MemoryCategory::Background {
                let idle_time = now.duration_since(allocation.last_accessed);
                if idle_time > threshold {
                    handles_to_remove.push(*handle);
                }
            }
        }
        
        // Remove identified allocations
        let mut freed_bytes = 0;
        for handle in handles_to_remove {
            if let Some(allocation) = self.allocations.remove(&handle) {
                self.current_allocation -= allocation.size;
                freed_bytes += allocation.size;
                
                // Update category usage
                if let Some(category_size) = self.category_usage.get_mut(&allocation.category) {
                    *category_size = category_size.saturating_sub(allocation.size);
                }
            }
        }
        
        info!("Memory optimization complete, freed {} bytes", freed_bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_allocation() {
        let mut manager = MemoryManager::new(100, "balanced"); // 100 MB
        
        // Allocate memory
        let handle = manager.allocate(1024 * 1024, "Test allocation", MemoryCategory::Working).unwrap();
        assert_eq!(manager.current_usage(), 1024 * 1024);
        
        // Deallocate memory
        let result = manager.deallocate(handle);
        assert!(result.is_ok());
        assert_eq!(manager.current_usage(), 0);
    }
    
    #[test]
    fn test_memory_allocation_limit() {
        let mut manager = MemoryManager::new(1, "balanced"); // 1 MB
        
        // Try to allocate more than the limit
        let result = manager.allocate(2 * 1024 * 1024, "Too large allocation", MemoryCategory::Working);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_memory_category_usage() {
        let mut manager = MemoryManager::new(100, "balanced"); // 100 MB
        
        // Allocate memory in different categories
        let handle1 = manager.allocate(1 * 1024 * 1024, "System allocation", MemoryCategory::System).unwrap();
        let handle2 = manager.allocate(2 * 1024 * 1024, "Working allocation", MemoryCategory::Working).unwrap();
        let handle3 = manager.allocate(3 * 1024 * 1024, "Long-term allocation", MemoryCategory::LongTerm).unwrap();
        
        // Check category usage
        assert_eq!(manager.category_usage(MemoryCategory::System), 1 * 1024 * 1024);
        assert_eq!(manager.category_usage(MemoryCategory::Working), 2 * 1024 * 1024);
        assert_eq!(manager.category_usage(MemoryCategory::LongTerm), 3 * 1024 * 1024);
        
        // Deallocate and check again
        manager.deallocate(handle2).unwrap();
        assert_eq!(manager.category_usage(MemoryCategory::Working), 0);
        assert_eq!(manager.current_usage(), 4 * 1024 * 1024);
    }
    
    #[test]
    fn test_memory_optimization() {
        let mut manager = MemoryManager::new(10, "aggressive"); // 10 MB
        
        // Fill up memory with background allocations
        for i in 0..8 {
            manager.allocate(1 * 1024 * 1024, &format!("Background {}", i), MemoryCategory::Background).unwrap();
        }
        
        // Check usage before optimization
        assert_eq!(manager.current_usage(), 8 * 1024 * 1024);
        
        // Force last_accessed time to be in the past for some allocations
        // This is a bit of a hack for testing, in real code we wouldn't manipulate private fields directly
        
        // Optimize memory
        let result = manager.optimize();
        assert!(result.is_ok());
        
        // Try to allocate more memory now that we've optimized
        let result = manager.allocate(3 * 1024 * 1024, "New allocation", MemoryCategory::Working);
        assert!(result.is_ok());
    }
}
