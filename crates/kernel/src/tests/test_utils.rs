//! Test utilities for kernel testing
//!
//! This module provides helper functions and mock implementations
//! to facilitate testing of kernel components.

use crate::Kernel;
use std::sync::{Arc, Mutex};

/// Create a test kernel instance with standard configuration
///
/// # Returns
///
/// A kernel instance ready for testing
pub fn create_test_kernel() -> Kernel {
    Kernel::new("test-version")
}

/// Create and initialize a test kernel
///
/// # Returns
///
/// An initialized kernel instance ready for testing
pub fn create_initialized_kernel() -> Result<Kernel, String> {
    let mut kernel = create_test_kernel();
    kernel.initialize()?;
    Ok(kernel)
}

/// Mock subsystem for testing kernel interactions
pub struct MockSubsystem {
    name: String,
    initialized: bool,
}

impl MockSubsystem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            initialized: false,
        }
    }
    
    pub fn initialize(&mut self) -> Result<(), String> {
        self.initialized = true;
        Ok(())
    }
    
    pub fn shutdown(&mut self) -> Result<(), String> {
        self.initialized = false;
        Ok(())
    }
    
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}
