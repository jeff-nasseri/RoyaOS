//! System call tests for the kernel
//!
//! This module tests the kernel's ability to process system calls
//! and route them to the appropriate subsystems.

use crate::Kernel;
use crate::tests::test_utils::create_initialized_kernel;

/// Test suite for system call processing
#[cfg(test)]
mod syscall_processing_tests {
    use super::*;
    
    /// Test processing of memory-related system calls
    #[test]
    fn test_memory_syscalls() {
        let kernel = match create_initialized_kernel() {
            Ok(k) => k,
            Err(e) => panic!("Failed to create initialized kernel: {}", e),
        };
        
        // Test memory allocation syscall
        let result = kernel.process_syscall("memory_alloc", &["1024", "heap"]);
        assert!(result.is_ok(), "Memory allocation syscall should succeed");
        
        // Test memory free syscall
        let result = kernel.process_syscall("memory_free", &["0x12345678"]);
        assert!(result.is_ok(), "Memory free syscall should succeed");
    }
    
    /// Test processing of tool-related system calls
    #[test]
    fn test_tool_syscalls() {
        let kernel = match create_initialized_kernel() {
            Ok(k) => k,
            Err(e) => panic!("Failed to create initialized kernel: {}", e),
        };
        
        // Test tool execution syscall
        let result = kernel.process_syscall("tool_execute", &["calculator", "add", "5", "3"]);
        assert!(result.is_ok(), "Tool execution syscall should succeed");
    }
    
    /// Test processing of security-related system calls
    #[test]
    fn test_security_syscalls() {
        let kernel = match create_initialized_kernel() {
            Ok(k) => k,
            Err(e) => panic!("Failed to create initialized kernel: {}", e),
        };
        
        // Test security check syscall
        let result = kernel.process_syscall("security_check", &["file", "read", "/tmp/test.txt"]);
        assert!(result.is_ok(), "Security check syscall should succeed");
    }
    
    /// Test handling of invalid system calls
    #[test]
    fn test_invalid_syscalls() {
        let kernel = match create_initialized_kernel() {
            Ok(k) => k,
            Err(e) => panic!("Failed to create initialized kernel: {}", e),
        };
        
        // Test non-existent syscall
        let result = kernel.process_syscall("nonexistent_syscall", &[]);
        assert!(result.is_err(), "Non-existent syscall should fail");
        
        // Test syscall with invalid arguments
        let result = kernel.process_syscall("memory_alloc", &[]);
        assert!(result.is_err(), "Syscall with invalid arguments should fail");
    }
}
