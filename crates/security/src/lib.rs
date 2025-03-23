//! Security module for RoyaOS
//!
//! This module handles security-related functionality for the RoyaOS system, including
//! access control, permission management, and security policy enforcement. It is designed
//! to ensure that Roya AGI operates within safe boundaries and cannot perform unauthorized
//! actions.
//!
//! The security system in RoyaOS provides:
//! - Fine-grained permission management
//! - Access control for system resources
//! - Security policy enforcement
//! - Audit logging for security events
//! - Threat detection and prevention

use log::{info, error, debug, warn};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Security level for the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Minimal security restrictions
    Low,
    /// Standard security restrictions
    Standard,
    /// High security restrictions
    High,
    /// Maximum security restrictions
    Maximum,
}

impl SecurityLevel {
    /// Convert a string to a SecurityLevel
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "low" => Ok(SecurityLevel::Low),
            "standard" => Ok(SecurityLevel::Standard),
            "high" => Ok(SecurityLevel::High),
            "maximum" => Ok(SecurityLevel::Maximum),
            _ => Err(format!("Invalid security level: {}", s)),
        }
    }
    
    /// Convert a SecurityLevel to a string
    pub fn as_str(&self) -> &'static str {
        match self {
            SecurityLevel::Low => "low",
            SecurityLevel::Standard => "standard",
            SecurityLevel::High => "high",
            SecurityLevel::Maximum => "maximum",
        }
    }
}

/// Permission representing an allowed operation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission {
    /// Resource type (file, network, memory, etc.)
    pub resource_type: String,
    /// Operation (read, write, execute, etc.)
    pub operation: String,
    /// Resource path or identifier
    pub resource: String,
}

/// Security event for audit logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event ID
    pub id: Uuid,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: String,
    /// Event source
    pub source: String,
    /// Event details
    pub details: String,
    /// Whether the event was allowed
    pub allowed: bool,
}

/// Security manager responsible for security-related functionality
#[derive(Debug)]
pub struct SecurityManager {
    /// Current security level
    security_level: SecurityLevel,
    /// Allowed permissions
    allowed_permissions: HashSet<Permission>,
    /// Security event log
    event_log: Vec<SecurityEvent>,
    /// Maximum event log size
    max_log_size: usize,
}

impl SecurityManager {
    /// Create a new security manager
    ///
    /// # Arguments
    ///
    /// * `security_level` - Security level for the system
    /// * `allowed_operations` - List of allowed operations
    ///
    /// # Returns
    ///
    /// A new SecurityManager instance
    pub fn new(security_level: &str, allowed_operations: Vec<String>) -> Result<Self, String> {
        let security_level = SecurityLevel::from_str(security_level)?;
        
        info!("Initializing security manager with {} security level", security_level.as_str());
        
        let mut allowed_permissions = HashSet::new();
        
        // Convert allowed operations to permissions
        for operation in allowed_operations {
            match operation.as_str() {
                "file_read" => {
                    allowed_permissions.insert(Permission {
                        resource_type: "file".to_string(),
                        operation: "read".to_string(),
                        resource: "*".to_string(),
                    });
                },
                "file_write" => {
                    allowed_permissions.insert(Permission {
                        resource_type: "file".to_string(),
                        operation: "write".to_string(),
                        resource: "*".to_string(),
                    });
                },
                "network_access" => {
                    allowed_permissions.insert(Permission {
                        resource_type: "network".to_string(),
                        operation: "connect".to_string(),
                        resource: "*".to_string(),
                    });
                },
                "tool_execution" => {
                    allowed_permissions.insert(Permission {
                        resource_type: "tool".to_string(),
                        operation: "execute".to_string(),
                        resource: "*".to_string(),
                    });
                },
                _ => {
                    warn!("Unknown operation: {}", operation);
                }
            }
        }
        
        Ok(Self {
            security_level,
            allowed_permissions,
            event_log: Vec::new(),
            max_log_size: 1000,
        })
    }
    
    /// Initialize the security manager
    ///
    /// # Returns
    ///
    /// `Ok(())` if initialization is successful, or an error message
    pub fn initialize(&mut self) -> Result<(), String> {
        info!("Initializing security manager");
        
        // Log initialization event
        self.log_event(
            "system",
            "security_initialization",
            &format!("Security manager initialized with {} security level", self.security_level.as_str()),
            true,
        );
        
        info!("Security manager initialization complete");
        Ok(())
    }
    
    /// Check if an operation is allowed
    ///
    /// # Arguments
    ///
    /// * `resource_type` - Type of resource being accessed
    /// * `operation` - Operation being performed
    /// * `resource` - Resource being accessed
    ///
    /// # Returns
    ///
    /// `true` if the operation is allowed, `false` otherwise
    pub fn check_permission(&mut self, resource_type: &str, operation: &str, resource: &str) -> bool {
        debug!("Checking permission: {} {} {}", resource_type, operation, resource);
        
        // Create permission to check
        let permission = Permission {
            resource_type: resource_type.to_string(),
            operation: operation.to_string(),
            resource: resource.to_string(),
        };
        
        // Check if permission is explicitly allowed
        let explicitly_allowed = self.allowed_permissions.contains(&permission);
        
        // Check if wildcard permission is allowed
        let wildcard_permission = Permission {
            resource_type: resource_type.to_string(),
            operation: operation.to_string(),
            resource: "*".to_string(),
        };
        let wildcard_allowed = self.allowed_permissions.contains(&wildcard_permission);
        
        // Determine if allowed based on security level and permissions
        let allowed = match self.security_level {
            SecurityLevel::Low => {
                // In low security, allow most operations
                true
            },
            SecurityLevel::Standard => {
                // In standard security, require explicit or wildcard permission
                explicitly_allowed || wildcard_allowed
            },
            SecurityLevel::High => {
                // In high security, require explicit permission
                explicitly_allowed
            },
            SecurityLevel::Maximum => {
                // In maximum security, require explicit permission and additional checks
                explicitly_allowed && self.additional_security_checks(resource_type, operation, resource)
            },
        };
        
        // Log the permission check
        self.log_event(
            "permission_check",
            &format!("{}_{}_{}", resource_type, operation, resource),
            &format!("Permission check for {} {} {}: {}", 
                    resource_type, operation, resource, if allowed { "allowed" } else { "denied" }),
            allowed,
        );
        
        allowed
    }
    
    /// Add a permission to the allowed permissions
    ///
    /// # Arguments
    ///
    /// * `resource_type` - Type of resource being accessed
    /// * `operation` - Operation being performed
    /// * `resource` - Resource being accessed
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error message
    pub fn add_permission(&mut self, resource_type: &str, operation: &str, resource: &str) -> Result<(), String> {
        info!("Adding permission: {} {} {}", resource_type, operation, resource);
        
        let permission = Permission {
            resource_type: resource_type.to_string(),
            operation: operation.to_string(),
            resource: resource.to_string(),
        };
        
        self.allowed_permissions.insert(permission);
        
        // Log the permission addition
        self.log_event(
            "permission_management",
            "add_permission",
            &format!("Added permission: {} {} {}", resource_type, operation, resource),
            true,
        );
        
        Ok(())
    }
    
    /// Remove a permission from the allowed permissions
    ///
    /// # Arguments
    ///
    /// * `resource_type` - Type of resource being accessed
    /// * `operation` - Operation being performed
    /// * `resource` - Resource being accessed
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error message
    pub fn remove_permission(&mut self, resource_type: &str, operation: &str, resource: &str) -> Result<(), String> {
        info!("Removing permission: {} {} {}", resource_type, operation, resource);
        
        let permission = Permission {
            resource_type: resource_type.to_string(),
            operation: operation.to_string(),
            resource: resource.to_string(),
        };
        
        self.allowed_permissions.remove(&permission);
        
        // Log the permission removal
        self.log_event(
            "permission_management",
            "remove_permission",
            &format!("Removed permission: {} {} {}", resource_type, operation, resource),
            true,
        );
        
        Ok(())
    }
    
    /// Set the security level
    ///
    /// # Arguments
    ///
    /// * `level` - New security level
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error message
    pub fn set_security_level(&mut self, level: &str) -> Result<(), String> {
        let new_level = SecurityLevel::from_str(level)?;
        
        info!("Changing security level from {} to {}", 
              self.security_level.as_str(), new_level.as_str());
        
        self.security_level = new_level;
        
        // Log the security level change
        self.log_event(
            "security_management",
            "change_security_level",
            &format!("Changed security level to {}", new_level.as_str()),
            true,
        );
        
        Ok(())
    }
    
    /// Get the current security level
    ///
    /// # Returns
    ///
    /// The current security level
    pub fn get_security_level(&self) -> SecurityLevel {
        self.security_level
    }
    
    /// Get recent security events
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of events to return
    ///
    /// # Returns
    ///
    /// Vector of recent security events
    pub fn get_recent_events(&self, limit: usize) -> Vec<SecurityEvent> {
        let start = if self.event_log.len() > limit {
            self.event_log.len() - limit
        } else {
            0
        };
        
        self.event_log[start..].to_vec()
    }
    
    /// Log a security event
    ///
    /// # Arguments
    ///
    /// * `source` - Event source
    /// * `event_type` - Event type
    /// * `details` - Event details
    /// * `allowed` - Whether the event was allowed
    fn log_event(&mut self, source: &str, event_type: &str, details: &str, allowed: bool) {
        let event = SecurityEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: event_type.to_string(),
            source: source.to_string(),
            details: details.to_string(),
            allowed,
        };
        
        // Add event to log
        self.event_log.push(event);
        
        // Trim log if it exceeds maximum size
        if self.event_log.len() > self.max_log_size {
            let excess = self.event_log.len() - self.max_log_size;
            self.event_log.drain(0..excess);
        }
    }
    
    /// Perform additional security checks for maximum security level
    ///
    /// # Arguments
    ///
    /// * `resource_type` - Type of resource being accessed
    /// * `operation` - Operation being performed
    /// * `resource` - Resource being accessed
    ///
    /// # Returns
    ///
    /// `true` if the operation passes additional checks, `false` otherwise
    fn additional_security_checks(&self, resource_type: &str, operation: &str, resource: &str) -> bool {
        // In a real implementation, this would perform additional security checks
        // For this example, we'll just implement some basic rules
        
        match resource_type {
            "file" => {
                // Don't allow access to system files
                if resource.starts_with("/system") || resource.starts_with("C:\\Windows") {
                    return false;
                }
                
                // Don't allow write to executable files
                if operation == "write" && (resource.ends_with(".exe") || resource.ends_with(".dll")) {
                    return false;
                }
                
                true
            },
            "network" => {
                // Only allow connections to specific domains or ports
                if operation == "connect" {
                    if resource.contains("localhost") || resource.contains("127.0.0.1") {
                        return true;
                    }
                    
                    // Allow connections to common API endpoints
                    if resource.contains("api.") {
                        return true;
                    }
                    
                    // Deny all other connections
                    return false;
                }
                
                true
            },
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_level_parsing() {
        assert_eq!(SecurityLevel::from_str("low").unwrap(), SecurityLevel::Low);
        assert_eq!(SecurityLevel::from_str("standard").unwrap(), SecurityLevel::Standard);
        assert_eq!(SecurityLevel::from_str("high").unwrap(), SecurityLevel::High);
        assert_eq!(SecurityLevel::from_str("maximum").unwrap(), SecurityLevel::Maximum);
        
        assert!(SecurityLevel::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_permission_checking() {
        let allowed_operations = vec![
            "file_read".to_string(),
            "network_access".to_string(),
        ];
        
        let mut manager = SecurityManager::new("standard", allowed_operations).unwrap();
        
        // Check allowed permissions
        assert!(manager.check_permission("file", "read", "test.txt"));
        assert!(manager.check_permission("network", "connect", "api.example.com"));
        
        // Check denied permissions
        assert!(!manager.check_permission("file", "write", "test.txt"));
        assert!(!manager.check_permission("tool", "execute", "calculator"));
    }
    
    #[test]
    fn test_permission_management() {
        let allowed_operations = vec![];
        let mut manager = SecurityManager::new("standard", allowed_operations).unwrap();
        
        // Initially denied
        assert!(!manager.check_permission("file", "read", "test.txt"));
        
        // Add permission
        manager.add_permission("file", "read", "test.txt").unwrap();
        
        // Now allowed
        assert!(manager.check_permission("file", "read", "test.txt"));
        
        // Remove permission
        manager.remove_permission("file", "read", "test.txt").unwrap();
        
        // Now denied again
        assert!(!manager.check_permission("file", "read", "test.txt"));
    }
    
    #[test]
    fn test_security_level_changes() {
        let allowed_operations = vec![
            "file_read".to_string(),
        ];
        
        let mut manager = SecurityManager::new("low", allowed_operations).unwrap();
        
        // In low security, most operations are allowed
        assert!(manager.check_permission("file", "write", "test.txt"));
        
        // Change to standard security
        manager.set_security_level("standard").unwrap();
        
        // In standard security, only explicitly allowed operations are permitted
        assert!(manager.check_permission("file", "read", "test.txt"));
        assert!(!manager.check_permission("file", "write", "test.txt"));
    }
}
