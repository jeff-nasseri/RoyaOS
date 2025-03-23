//! Configuration module for RoyaOS
//!
//! This module handles loading and managing system configuration.

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::RoyaOsError;

/// Main configuration structure for RoyaOS
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// System configuration
    pub system: SystemConfig,
    /// Memory configuration
    pub memory: MemoryConfig,
    /// Tools configuration
    pub tools: ToolsConfig,
    /// Security configuration
    pub security: SecurityConfig,
}

/// System configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfig {
    /// System name
    pub name: String,
    /// System version
    pub version: String,
    /// Log level
    pub log_level: String,
    /// Data directory
    pub data_dir: String,
}

/// Memory configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum memory allocation (in MB)
    pub max_allocation: usize,
    /// Memory optimization strategy
    pub optimization_strategy: String,
}

/// Tools configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsConfig {
    /// Tool discovery enabled
    pub discovery_enabled: bool,
    /// Tool directories
    pub tool_dirs: Vec<String>,
}

/// Security configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Security level
    pub security_level: String,
    /// Allowed operations
    pub allowed_operations: Vec<String>,
}

/// Load configuration from file
pub fn load_config() -> Result<Config, RoyaOsError> {
    let config_path = Path::new("config/config.yaml");
    
    // Check if config file exists
    if !config_path.exists() {
        return Err(RoyaOsError::ConfigNotFound);
    }
    
    // Open and read config file
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // Parse YAML
    let config: Config = serde_yaml::from_str(&contents)?;
    
    Ok(config)
}
