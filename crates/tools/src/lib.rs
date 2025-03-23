//! Tool management module for RoyaOS
//!
//! This module handles the discovery, integration, and execution of tools for the RoyaOS system.
//! Tools are external capabilities that can be leveraged by the Roya AGI to interact with the
//! world, process data, or perform specialized tasks.
//!
//! The tool system in RoyaOS provides:
//! - Tool discovery and registration
//! - Tool execution and result handling
//! - Tool permission management
//! - Tool versioning and compatibility checking

use log::{info, error, debug, warn};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Tool handle type used to reference registered tools
pub type ToolHandle = Uuid;

/// Tool capability representing a specific function a tool can perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapability {
    /// Name of the capability
    pub name: String,
    /// Description of what the capability does
    pub description: String,
    /// Required parameters for the capability
    pub parameters: Vec<ToolParameter>,
    /// Return type of the capability
    pub return_type: String,
}

/// Tool parameter for capability execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    /// Name of the parameter
    pub name: String,
    /// Description of the parameter
    pub description: String,
    /// Type of the parameter (string, number, boolean, etc.)
    pub param_type: String,
    /// Whether the parameter is required
    pub required: bool,
    /// Default value for the parameter (if any)
    pub default_value: Option<String>,
}

/// Tool metadata containing information about a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// Unique identifier for the tool
    pub id: String,
    /// Name of the tool
    pub name: String,
    /// Description of the tool
    pub description: String,
    /// Version of the tool
    pub version: String,
    /// Author of the tool
    pub author: String,
    /// Categories the tool belongs to
    pub categories: Vec<String>,
    /// Capabilities provided by the tool
    pub capabilities: Vec<ToolCapability>,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Whether the execution was successful
    pub success: bool,
    /// Result data (if successful)
    pub data: Option<String>,
    /// Error message (if unsuccessful)
    pub error: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Tool instance representing a registered tool
#[derive(Debug)]
struct ToolInstance {
    /// Tool metadata
    metadata: ToolMetadata,
    /// Path to the tool executable or library
    path: PathBuf,
    /// Whether the tool is currently enabled
    enabled: bool,
    /// Number of times the tool has been executed
    execution_count: usize,
    /// Last execution time
    last_execution: Option<std::time::Instant>,
}

/// Tool manager responsible for managing tools in RoyaOS
#[derive(Debug)]
pub struct ToolManager {
    /// Map of tool handles to tool instances
    tools: HashMap<ToolHandle, ToolInstance>,
    /// Tool directories to search for tools
    tool_dirs: Vec<PathBuf>,
    /// Whether tool discovery is enabled
    discovery_enabled: bool,
    /// Tool execution history
    execution_history: Vec<(ToolHandle, std::time::Instant, bool)>,
}

impl ToolManager {
    /// Create a new tool manager
    ///
    /// # Arguments
    ///
    /// * `tool_dirs` - Directories to search for tools
    /// * `discovery_enabled` - Whether tool discovery is enabled
    ///
    /// # Returns
    ///
    /// A new ToolManager instance
    pub fn new(tool_dirs: Vec<String>, discovery_enabled: bool) -> Self {
        let tool_dirs = tool_dirs.iter()
            .map(|dir| PathBuf::from(dir))
            .collect();
        
        info!("Initializing tool manager with discovery {}", 
              if discovery_enabled { "enabled" } else { "disabled" });
        
        Self {
            tools: HashMap::new(),
            tool_dirs,
            discovery_enabled,
            execution_history: Vec::new(),
        }
    }
    
    /// Initialize the tool manager
    ///
    /// This method discovers and registers available tools.
    ///
    /// # Returns
    ///
    /// `Ok(())` if initialization is successful, or an error message
    pub fn initialize(&mut self) -> Result<(), String> {
        info!("Initializing tool manager");
        
        if self.discovery_enabled {
            self.discover_tools()?;
        }
        
        info!("Tool manager initialization complete, {} tools registered", self.tools.len());
        Ok(())
    }
    
    /// Discover tools in the configured tool directories
    ///
    /// # Returns
    ///
    /// `Ok(())` if discovery is successful, or an error message
    pub fn discover_tools(&mut self) -> Result<(), String> {
        info!("Discovering tools in {} directories", self.tool_dirs.len());
        
        for dir in &self.tool_dirs {
            debug!("Searching for tools in directory: {:?}", dir);
            
            if !dir.exists() {
                warn!("Tool directory does not exist: {:?}", dir);
                continue;
            }
            
            // In a real implementation, we would scan the directory for tool manifests
            // and load them. For this example, we'll just simulate finding tools.
            
            // Simulate finding a calculator tool
            let calculator_metadata = ToolMetadata {
                id: "calculator".to_string(),
                name: "Calculator".to_string(),
                description: "Performs mathematical calculations".to_string(),
                version: "1.0.0".to_string(),
                author: "RoyaOS Team".to_string(),
                categories: vec!["math".to_string(), "utility".to_string()],
                capabilities: vec![
                    ToolCapability {
                        name: "add".to_string(),
                        description: "Add two numbers".to_string(),
                        parameters: vec![
                            ToolParameter {
                                name: "a".to_string(),
                                description: "First number".to_string(),
                                param_type: "number".to_string(),
                                required: true,
                                default_value: None,
                            },
                            ToolParameter {
                                name: "b".to_string(),
                                description: "Second number".to_string(),
                                param_type: "number".to_string(),
                                required: true,
                                default_value: None,
                            },
                        ],
                        return_type: "number".to_string(),
                    },
                    ToolCapability {
                        name: "subtract".to_string(),
                        description: "Subtract two numbers".to_string(),
                        parameters: vec![
                            ToolParameter {
                                name: "a".to_string(),
                                description: "First number".to_string(),
                                param_type: "number".to_string(),
                                required: true,
                                default_value: None,
                            },
                            ToolParameter {
                                name: "b".to_string(),
                                description: "Second number".to_string(),
                                param_type: "number".to_string(),
                                required: true,
                                default_value: None,
                            },
                        ],
                        return_type: "number".to_string(),
                    },
                ],
            };
            
            let calculator_path = dir.join("calculator");
            self.register_tool(calculator_metadata, calculator_path)?;
        }
        
        Ok(())
    }
    
    /// Register a tool with the tool manager
    ///
    /// # Arguments
    ///
    /// * `metadata` - Tool metadata
    /// * `path` - Path to the tool executable or library
    ///
    /// # Returns
    ///
    /// Handle to the registered tool, or an error message
    pub fn register_tool(&mut self, metadata: ToolMetadata, path: PathBuf) -> Result<ToolHandle, String> {
        info!("Registering tool: {} ({})", metadata.name, metadata.id);
        
        let handle = Uuid::new_v4();
        let tool = ToolInstance {
            metadata,
            path,
            enabled: true,
            execution_count: 0,
            last_execution: None,
        };
        
        self.tools.insert(handle, tool);
        debug!("Tool registered with handle {}", handle);
        
        Ok(handle)
    }
    
    /// Execute a tool capability
    ///
    /// # Arguments
    ///
    /// * `handle` - Handle to the tool
    /// * `capability` - Name of the capability to execute
    /// * `params` - Parameters for the capability
    ///
    /// # Returns
    ///
    /// Result of the tool execution, or an error message
    pub fn execute_tool(&mut self, handle: ToolHandle, capability: &str, params: &str) -> Result<ToolResult, String> {
        debug!("Executing tool {} capability {} with params {}", handle, capability, params);
        
        let tool = self.tools.get_mut(&handle).ok_or_else(|| {
            let error_msg = format!("No tool found for handle {}", handle);
            error!("{}", error_msg);
            error_msg
        })?;
        
        if !tool.enabled {
            let error_msg = format!("Tool {} is disabled", handle);
            error!("{}", error_msg);
            return Err(error_msg);
        }
        
        // Find the capability
        let capability_info = tool.metadata.capabilities.iter()
            .find(|cap| cap.name == capability)
            .ok_or_else(|| {
                let error_msg = format!("Capability {} not found for tool {}", capability, handle);
                error!("{}", error_msg);
                error_msg
            })?;
        
        // In a real implementation, we would actually execute the tool
        // For this example, we'll just simulate execution
        
        let start_time = std::time::Instant::now();
        
        // Simulate execution
        let result = match capability {
            "add" => {
                // Parse parameters
                let params: serde_json::Value = serde_json::from_str(params)
                    .map_err(|e| format!("Failed to parse parameters: {}", e))?;
                
                let a = params["a"].as_f64().ok_or("Parameter 'a' must be a number")?;
                let b = params["b"].as_f64().ok_or("Parameter 'b' must be a number")?;
                
                let sum = a + b;
                
                ToolResult {
                    success: true,
                    data: Some(sum.to_string()),
                    error: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                }
            },
            "subtract" => {
                // Parse parameters
                let params: serde_json::Value = serde_json::from_str(params)
                    .map_err(|e| format!("Failed to parse parameters: {}", e))?;
                
                let a = params["a"].as_f64().ok_or("Parameter 'a' must be a number")?;
                let b = params["b"].as_f64().ok_or("Parameter 'b' must be a number")?;
                
                let difference = a - b;
                
                ToolResult {
                    success: true,
                    data: Some(difference.to_string()),
                    error: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                }
            },
            _ => {
                let error_msg = format!("Capability {} not implemented", capability);
                error!("{}", error_msg);
                
                ToolResult {
                    success: false,
                    data: None,
                    error: Some(error_msg),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                }
            }
        };
        
        // Update tool statistics
        tool.execution_count += 1;
        tool.last_execution = Some(start_time);
        
        // Record in execution history
        self.execution_history.push((handle, start_time, result.success));
        
        Ok(result)
    }
    
    /// Get a list of all registered tools
    ///
    /// # Returns
    ///
    /// Vector of tool handles and their metadata
    pub fn list_tools(&self) -> Vec<(ToolHandle, ToolMetadata)> {
        self.tools.iter()
            .map(|(handle, tool)| (*handle, tool.metadata.clone()))
            .collect()
    }
    
    /// Get information about a specific tool
    ///
    /// # Arguments
    ///
    /// * `handle` - Handle to the tool
    ///
    /// # Returns
    ///
    /// Tool metadata, or an error message
    pub fn get_tool_info(&self, handle: ToolHandle) -> Result<ToolMetadata, String> {
        let tool = self.tools.get(&handle).ok_or_else(|| {
            let error_msg = format!("No tool found for handle {}", handle);
            error!("{}", error_msg);
            error_msg
        })?;
        
        Ok(tool.metadata.clone())
    }
    
    /// Enable or disable a tool
    ///
    /// # Arguments
    ///
    /// * `handle` - Handle to the tool
    /// * `enabled` - Whether the tool should be enabled
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error message
    pub fn set_tool_enabled(&mut self, handle: ToolHandle, enabled: bool) -> Result<(), String> {
        let tool = self.tools.get_mut(&handle).ok_or_else(|| {
            let error_msg = format!("No tool found for handle {}", handle);
            error!("{}", error_msg);
            error_msg
        })?;
        
        tool.enabled = enabled;
        info!("Tool {} {} {}", handle, tool.metadata.name, if enabled { "enabled" } else { "disabled" });
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_registration() {
        let mut manager = ToolManager::new(vec!["./tools".to_string()], true);
        
        let metadata = ToolMetadata {
            id: "test-tool".to_string(),
            name: "Test Tool".to_string(),
            description: "A tool for testing".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            categories: vec!["test".to_string()],
            capabilities: vec![],
        };
        
        let path = PathBuf::from("./tools/test-tool");
        let handle = manager.register_tool(metadata.clone(), path).unwrap();
        
        let tools = manager.list_tools();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].1.id, "test-tool");
    }
    
    #[test]
    fn test_tool_execution() {
        let mut manager = ToolManager::new(vec!["./tools".to_string()], true);
        
        // Register a calculator tool
        let metadata = ToolMetadata {
            id: "calculator".to_string(),
            name: "Calculator".to_string(),
            description: "Performs mathematical calculations".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            categories: vec!["math".to_string()],
            capabilities: vec![
                ToolCapability {
                    name: "add".to_string(),
                    description: "Add two numbers".to_string(),
                    parameters: vec![
                        ToolParameter {
                            name: "a".to_string(),
                            description: "First number".to_string(),
                            param_type: "number".to_string(),
                            required: true,
                            default_value: None,
                        },
                        ToolParameter {
                            name: "b".to_string(),
                            description: "Second number".to_string(),
                            param_type: "number".to_string(),
                            required: true,
                            default_value: None,
                        },
                    ],
                    return_type: "number".to_string(),
                },
            ],
        };
        
        let path = PathBuf::from("./tools/calculator");
        let handle = manager.register_tool(metadata, path).unwrap();
        
        // Execute the add capability
        let params = r#"{"a": 2, "b": 3}"#;
        let result = manager.execute_tool(handle, "add", params).unwrap();
        
        assert!(result.success);
        assert_eq!(result.data, Some("5".to_string()));
    }
}
