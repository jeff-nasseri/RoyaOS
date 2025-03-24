//! Core kernel functionality tests
//!
//! This module contains tests for the basic kernel functionality,
//! including initialization, shutdown, and state management.

use crate::Kernel;
use crate::tests::test_utils::{create_test_kernel, create_initialized_kernel};

/// Test suite for kernel lifecycle operations
#[cfg(test)]
mod lifecycle_tests {
    use super::*;
    
    /// Test kernel creation with proper version
    #[test]
    fn test_kernel_creation() {
        let version = "1.0.0-test";
        let kernel = Kernel::new(version);
        
        assert_eq!(kernel.version(), version, "Kernel should have the correct version");
        assert_eq!(kernel.is_running(), false, "New kernel should not be running");
    }
    
    /// Test complete kernel initialization sequence
    #[test]
    fn test_kernel_initialization() {
        let mut kernel = create_test_kernel();
        
        // Kernel should start in non-running state
        assert_eq!(kernel.is_running(), false, "Kernel should start in non-running state");
        
        // Initialize the kernel
        let init_result = kernel.initialize();
        assert!(init_result.is_ok(), "Kernel initialization should succeed");
        assert_eq!(kernel.is_running(), true, "Kernel should be running after initialization");
        
        // Verify subsystems are registered
        // This would require exposing subsystem state or adding a method to check
    }
    
    /// Test kernel shutdown sequence
    #[test]
    fn test_kernel_shutdown() {
        // Create and initialize kernel
        let mut kernel = match create_initialized_kernel() {
            Ok(k) => k,
            Err(e) => panic!("Failed to create initialized kernel: {}", e),
        };
        
        // Kernel should be running after initialization
        assert_eq!(kernel.is_running(), true, "Kernel should be running after initialization");
        
        // Shutdown the kernel
        let shutdown_result = kernel.shutdown();
        assert!(shutdown_result.is_ok(), "Kernel shutdown should succeed");
        assert_eq!(kernel.is_running(), false, "Kernel should not be running after shutdown");
    }
    
    /// Test kernel initialization and shutdown multiple times
    #[test]
    fn test_kernel_restart() {
        let mut kernel = create_test_kernel();
        
        // First cycle
        assert!(kernel.initialize().is_ok(), "First initialization should succeed");
        assert_eq!(kernel.is_running(), true, "Kernel should be running after first init");
        assert!(kernel.shutdown().is_ok(), "First shutdown should succeed");
        assert_eq!(kernel.is_running(), false, "Kernel should not be running after first shutdown");
        
        // Second cycle
        assert!(kernel.initialize().is_ok(), "Second initialization should succeed");
        assert_eq!(kernel.is_running(), true, "Kernel should be running after second init");
        assert!(kernel.shutdown().is_ok(), "Second shutdown should succeed");
        assert_eq!(kernel.is_running(), false, "Kernel should not be running after second shutdown");
    }
}

/// Test suite for kernel version management
#[cfg(test)]
mod version_tests {
    use super::*;
    
    /// Test kernel version reporting
    #[test]
    fn test_kernel_version() {
        let test_versions = vec!["0.1.0", "1.0.0", "2.3.4-alpha", "0.0.1-dev"];
        
        for version in test_versions {
            let kernel = Kernel::new(version);
            assert_eq!(kernel.version(), version, "Kernel should report correct version");
        }
    }
}
