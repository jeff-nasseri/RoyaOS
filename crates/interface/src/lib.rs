//! Interface module for RoyaOS
//!
//! This module provides the interface layer between Roya AGI and the RoyaOS kernel.
//! It handles communication, request processing, and response formatting to enable
//! seamless interaction between the AGI and the operating system.
//!
//! The interface system in RoyaOS provides:
//! - API endpoints for AGI interaction
//! - Request validation and processing
//! - Response formatting and delivery
//! - Session management
//! - Interface versioning and compatibility

use log::{info, error, debug, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Session handle type used to reference AGI sessions
pub type SessionHandle = Uuid;

/// Request from Roya AGI to the RoyaOS kernel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// Request ID
    pub id: String,
    /// Request type
    pub request_type: String,
    /// Request parameters
    pub parameters: serde_json::Value,
    /// Request timestamp
    pub timestamp: u64,
}

/// Response from the RoyaOS kernel to Roya AGI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Response ID (matches request ID)
    pub id: String,
    /// Whether the request was successful
    pub success: bool,
    /// Response data (if successful)
    pub data: Option<serde_json::Value>,
    /// Error message (if unsuccessful)
    pub error: Option<String>,
    /// Response timestamp
    pub timestamp: u64,
}

/// Session representing an active connection from Roya AGI
#[derive(Debug)]
struct Session {
    /// Session ID
    id: SessionHandle,
    /// Session creation time
    created_at: std::time::Instant,
    /// Last activity time
    last_activity: std::time::Instant,
    /// Session metadata
    metadata: HashMap<String, String>,
}

/// Interface manager responsible for handling AGI-OS communication
#[derive(Debug)]
pub struct InterfaceManager {
    /// Active sessions
    sessions: HashMap<SessionHandle, Session>,
    /// API version
    api_version: String,
    /// Request handlers
    request_handlers: HashMap<String, Box<dyn Fn(&Request) -> Response + Send + Sync>>,
}

impl InterfaceManager {
    /// Create a new interface manager
    ///
    /// # Arguments
    ///
    /// * `api_version` - API version string
    ///
    /// # Returns
    ///
    /// A new InterfaceManager instance
    pub fn new(api_version: &str) -> Self {
        info!("Initializing interface manager with API version {}", api_version);
        
        Self {
            sessions: HashMap::new(),
            api_version: api_version.to_string(),
            request_handlers: HashMap::new(),
        }
    }
    
    /// Initialize the interface manager
    ///
    /// This method sets up request handlers and prepares the interface for operation.
    ///
    /// # Returns
    ///
    /// `Ok(())` if initialization is successful, or an error message
    pub fn initialize(&mut self) -> Result<(), String> {
        info!("Initializing interface manager");
        
        // Register default request handlers
        self.register_default_handlers();
        
        info!("Interface manager initialization complete");
        Ok(())
    }
    
    /// Create a new session for Roya AGI
    ///
    /// # Arguments
    ///
    /// * `metadata` - Session metadata
    ///
    /// # Returns
    ///
    /// Handle to the new session
    pub fn create_session(&mut self, metadata: HashMap<String, String>) -> SessionHandle {
        let session_id = Uuid::new_v4();
        let now = std::time::Instant::now();
        
        let session = Session {
            id: session_id,
            created_at: now,
            last_activity: now,
            metadata,
        };
        
        self.sessions.insert(session_id, session);
        info!("Created new session with ID {}", session_id);
        
        session_id
    }
    
    /// Close a session
    ///
    /// # Arguments
    ///
    /// * `session_id` - ID of the session to close
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error message
    pub fn close_session(&mut self, session_id: SessionHandle) -> Result<(), String> {
        if self.sessions.remove(&session_id).is_some() {
            info!("Closed session with ID {}", session_id);
            Ok(())
        } else {
            let error_msg = format!("Session {} not found", session_id);
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
    
    /// Process a request from Roya AGI
    ///
    /// # Arguments
    ///
    /// * `session_id` - ID of the session making the request
    /// * `request` - Request to process
    ///
    /// # Returns
    ///
    /// Response to the request, or an error message
    pub fn process_request(&mut self, session_id: SessionHandle, request: Request) -> Result<Response, String> {
        debug!("Processing request {} of type {} for session {}", 
               request.id, request.request_type, session_id);
        
        // Update session activity
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.last_activity = std::time::Instant::now();
        } else {
            let error_msg = format!("Session {} not found", session_id);
            error!("{}", error_msg);
            return Err(error_msg);
        }
        
        // Find handler for request type
        if let Some(handler) = self.request_handlers.get(&request.request_type) {
            let response = handler(&request);
            Ok(response)
        } else {
            let error_msg = format!("No handler found for request type {}", request.request_type);
            error!("{}", error_msg);
            
            let response = Response {
                id: request.id,
                success: false,
                data: None,
                error: Some(error_msg),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            };
            
            Ok(response)
        }
    }
    
    /// Register a request handler
    ///
    /// # Arguments
    ///
    /// * `request_type` - Type of request to handle
    /// * `handler` - Function to handle the request
    ///
    /// # Returns
    ///
    /// `Ok(())` if registration is successful, or an error message
    pub fn register_handler<F>(&mut self, request_type: &str, handler: F) -> Result<(), String>
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        info!("Registering handler for request type {}", request_type);
        
        self.request_handlers.insert(request_type.to_string(), Box::new(handler));
        
        Ok(())
    }
    
    /// Get the API version
    ///
    /// # Returns
    ///
    /// The API version string
    pub fn get_api_version(&self) -> &str {
        &self.api_version
    }
    
    /// Get active sessions
    ///
    /// # Returns
    ///
    /// Vector of active session IDs
    pub fn get_active_sessions(&self) -> Vec<SessionHandle> {
        self.sessions.keys().cloned().collect()
    }
    
    /// Register default request handlers
    fn register_default_handlers(&mut self) {
        // Register system info handler
        self.register_handler("system_info", |request| {
            let system_info = serde_json::json!({
                "name": "RoyaOS",
                "version": "0.1.0",
                "api_version": "1.0",
                "uptime_seconds": 0, // In a real implementation, this would be the actual uptime
            });
            
            Response {
                id: request.id.clone(),
                success: true,
                data: Some(system_info),
                error: None,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            }
        }).unwrap();
        
        // Register echo handler (for testing)
        self.register_handler("echo", |request| {
            Response {
                id: request.id.clone(),
                success: true,
                data: Some(request.parameters.clone()),
                error: None,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            }
        }).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_management() {
        let mut manager = InterfaceManager::new("1.0");
        
        // Create session
        let metadata = HashMap::new();
        let session_id = manager.create_session(metadata);
        
        // Check active sessions
        let sessions = manager.get_active_sessions();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0], session_id);
        
        // Close session
        let result = manager.close_session(session_id);
        assert!(result.is_ok());
        
        // Check active sessions again
        let sessions = manager.get_active_sessions();
        assert_eq!(sessions.len(), 0);
    }
    
    #[test]
    fn test_request_processing() {
        let mut manager = InterfaceManager::new("1.0");
        manager.initialize().unwrap();
        
        // Create session
        let metadata = HashMap::new();
        let session_id = manager.create_session(metadata);
        
        // Create request
        let request = Request {
            id: "test-request".to_string(),
            request_type: "echo".to_string(),
            parameters: serde_json::json!({
                "message": "Hello, RoyaOS!",
            }),
            timestamp: 0,
        };
        
        // Process request
        let result = manager.process_request(session_id, request);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert_eq!(response.id, "test-request");
        
        // Check response data
        let data = response.data.unwrap();
        assert_eq!(data["message"], "Hello, RoyaOS!");
    }
    
    #[test]
    fn test_custom_handler() {
        let mut manager = InterfaceManager::new("1.0");
        
        // Register custom handler
        manager.register_handler("custom", |request| {
            Response {
                id: request.id.clone(),
                success: true,
                data: Some(serde_json::json!({
                    "result": "Custom handler executed",
                })),
                error: None,
                timestamp: 0,
            }
        }).unwrap();
        
        // Create session
        let metadata = HashMap::new();
        let session_id = manager.create_session(metadata);
        
        // Create request
        let request = Request {
            id: "custom-request".to_string(),
            request_type: "custom".to_string(),
            parameters: serde_json::json!({}),
            timestamp: 0,
        };
        
        // Process request
        let result = manager.process_request(session_id, request);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        
        // Check response data
        let data = response.data.unwrap();
        assert_eq!(data["result"], "Custom handler executed");
    }
}
