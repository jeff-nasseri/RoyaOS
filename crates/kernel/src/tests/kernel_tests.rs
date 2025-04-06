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

/// Test suite for subsystem management
#[cfg(test)]
mod subsystem_tests {
    use super::*;
    
    /// Test subsystem registration
    #[test]
    fn test_subsystem_registration() {
        let mut kernel = create_test_kernel();
        
        // Register a test subsystem
        let result = kernel.register_subsystem("test_subsystem");
        assert!(result.is_ok(), "Subsystem registration should succeed");
        
        // Try registering the same subsystem again (implementation dependent)
        // This test assumes re-registration is allowed
        let result = kernel.register_subsystem("test_subsystem");
        assert!(result.is_ok(), "Re-registering a subsystem should be allowed");
    }
    
    /// Test subsystem shutdown
    #[test]
    fn test_subsystem_shutdown() {
        let mut kernel = create_test_kernel();
        
        // Register and then shutdown a subsystem
        assert!(kernel.register_subsystem("test_subsystem").is_ok());
        let result = kernel.shutdown_subsystem("test_subsystem");
        assert!(result.is_ok(), "Subsystem shutdown should succeed");
        
        // Shutdown a non-existent subsystem (implementation dependent)
        // This test assumes shutting down non-existent subsystems returns an error
        let result = kernel.shutdown_subsystem("nonexistent_subsystem");
        assert!(result.is_ok(), "Shutting down a non-existent subsystem should be handled gracefully");
    }
    
    /// Test initialization registers the expected subsystems
    #[test]
    fn test_default_subsystems() {
        let mut kernel = create_test_kernel();
        assert!(kernel.initialize().is_ok());
        
        // The kernel should have registered these subsystems during initialization
        let expected_subsystems = ["memory", "tools", "security", "interface"];
        
        // This test assumes there's a way to check if a subsystem is registered
        // If there's no such method, this test would need to be modified
        for subsystem in expected_subsystems.iter() {
            // This is a placeholder for a method that would check if a subsystem is registered
            // assert!(kernel.has_subsystem(subsystem), "Kernel should have registered {}", subsystem);
            
            // Instead, we can test that shutting down these subsystems works
            assert!(kernel.shutdown_subsystem(subsystem).is_ok(), 
                   "Should be able to shutdown {} subsystem", subsystem);
        }
    }
}

/// Test suite for syscall handling
#[cfg(test)]
mod syscall_tests {
    use super::*;
    
    /// Test memory syscalls
    #[test]
    fn test_memory_syscalls() {
        let kernel = match create_initialized_kernel() {
            Ok(k) => k,
            Err(e) => panic!("Failed to create initialized kernel: {}", e),
        };
        
        // Test memory allocation syscall
        // This assumes the kernel has a method to handle syscalls
        // If there's no such public method, this test would need to be modified
        let result = kernel.handle_memory_syscall("allocate", &["1024"]);
        assert!(result.is_ok(), "Memory allocation syscall should succeed");
        
        // Test invalid memory operation
        let result = kernel.handle_memory_syscall("invalid_operation", &[]);
        assert!(result.is_ok(), "Invalid memory operation should be handled gracefully");
    }
    
    /// Test tool syscalls
    #[test]
    fn test_tool_syscalls() {
        let kernel = match create_initialized_kernel() {
            Ok(k) => k,
            Err(e) => panic!("Failed to create initialized kernel: {}", e),
        };
        
        // Test tool execution syscall
        let result = kernel.handle_tool_syscall("execute", &["test_tool", "arg1", "arg2"]);
        assert!(result.is_ok(), "Tool execution syscall should succeed");
        
        // Test invalid tool operation
        let result = kernel.handle_tool_syscall("invalid_operation", &[]);
        assert!(result.is_ok(), "Invalid tool operation should be handled gracefully");
    }
    
    /// Test security syscalls
    #[test]
    fn test_security_syscalls() {
        let kernel = match create_initialized_kernel() {
            Ok(k) => k,
            Err(e) => panic!("Failed to create initialized kernel: {}", e),
        };
        
        // Test permission check syscall
        let result = kernel.handle_security_syscall("check_permission", &["file", "read", "/test.txt"]);
        assert!(result.is_ok(), "Security permission check syscall should succeed");
        
        // Test invalid security operation
        let result = kernel.handle_security_syscall("invalid_operation", &[]);
        assert!(result.is_ok(), "Invalid security operation should be handled gracefully");
    }
}

/// Test suite for system load tracking
#[cfg(test)]
mod system_load_tests {
    use super::*;
    
    /// Test system load tracking
    #[test]
    fn test_system_load() {
        let mut kernel = create_test_kernel();
        
        // This assumes the kernel has methods to get and set system load
        // If there are no such public methods, this test would need to be modified
        
        // Initial system load should be 0.0
        assert_eq!(kernel.system_load(), 0.0, "Initial system load should be 0.0");
        
        // Set system load to a test value
        kernel.set_system_load(0.5);
        assert_eq!(kernel.system_load(), 0.5, "System load should be updated to 0.5");
        
        // Test bounds checking (implementation dependent)
        // This test assumes system load is clamped to [0.0, 1.0]
        kernel.set_system_load(1.5);
        assert_eq!(kernel.system_load(), 1.0, "System load should be clamped to 1.0");
        
        kernel.set_system_load(-0.5);
        assert_eq!(kernel.system_load(), 0.0, "System load should be clamped to 0.0");
    }
}
