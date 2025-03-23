//! Error types for RoyaOS
//!
//! This module defines the error types used throughout the system.

use thiserror::Error;

/// Main error type for RoyaOS
#[derive(Error, Debug)]
pub enum RoyaOsError {
    /// Configuration not found
    #[error("Configuration file not found")]
    ConfigNotFound,
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// YAML parsing error
    #[error("YAML parsing error: {0}")]
    YamlParsing(#[from] serde_yaml::Error),
    
    /// Memory allocation error
    #[error("Memory allocation error: {0}")]
    MemoryAllocation(String),
    
    /// Tool error
    #[error("Tool error: {0}")]
    Tool(String),
    
    /// Security error
    #[error("Security error: {0}")]
    Security(String),
    
    /// Interface error
    #[error("Interface error: {0}")]
    Interface(String),
    
    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}
