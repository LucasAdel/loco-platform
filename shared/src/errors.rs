use thiserror::Error;
use serde::{Deserialize, Serialize};

/// Comprehensive error types for the Loco Platform
/// Following Australian English conventions and providing detailed context
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    // Database errors with context preservation
    #[error("Database error: {0}")]
    Database(String),
    
    // Database errors with additional context
    #[error("Database error: {message} (Context: {context})")]
    DatabaseWithContext {
        message: String,
        context: String,
    },
    
    // Validation errors with field-specific details
    #[error("Validation error in {field}: {message}")]
    Validation {
        field: String,
        message: String,
        value: Option<String>,
    },
    
    // Authentication errors with user-friendly messages
    #[error("Authentication failed: {reason}")]
    Authentication {
        reason: String,
        retry_allowed: bool,
    },
    
    // Authorisation errors (Australian spelling)
    #[error("Authorisation denied: {resource} requires {permission}")]
    Authorisation {
        resource: String,
        permission: String,
        user_id: Option<String>,
    },
    
    // Simple not found error
    #[error("Resource not found")]
    NotFound,
    
    // Not found errors with helpful suggestions
    #[error("Resource not found: {resource_type} with ID {id}")]
    NotFoundDetailed {
        resource_type: String,
        id: String,
        suggestions: Vec<String>,
    },
    
    // Internal server errors with tracking
    #[error("Internal error: {message} (Error ID: {error_id})")]
    Internal {
        message: String,
        error_id: String,
        component: String,
    },
    
    // External service errors with retry information
    #[error("External service error: {service} - {message}")]
    ExternalService {
        service: String,
        message: String,
        status_code: Option<u16>,
        retry_after: Option<u64>,
    },
    
    // Rate limiting with Australian spelling
    #[error("Rate limit exceeded for {resource}. Try again in {retry_after_seconds} seconds")]
    RateLimit {
        resource: String,
        retry_after_seconds: u64,
        limit: u32,
    },
    
    // Input validation with detailed feedback
    #[error("Invalid input: {message}")]
    InvalidInput {
        message: String,
        field: Option<String>,
        expected_format: Option<String>,
    },
    
    // Network/connectivity errors
    #[error("Network error: {message}")]
    Network {
        message: String,
        timeout: bool,
        retry_count: u32,
    },
    
    // Business logic errors
    #[error("Business rule violation: {rule}")]
    BusinessRule {
        rule: String,
        message: String,
        severity: ErrorSeverity,
    },
    
    // File/IO errors
    #[error("File operation failed: {operation} on {path}")]
    FileOperation {
        operation: String,
        path: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Info,
    Warning, 
    Error,
    Critical,
}

impl AppError {
    /// Create a database error with context
    pub fn database(message: impl Into<String>, context: impl Into<String>) -> Self {
        Self::DatabaseWithContext {
            message: message.into(),
            context: context.into(),
        }
    }
    
    /// Create a validation error for a specific field
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
            value: None,
        }
    }
    
    /// Create a not found error with suggestions
    pub fn not_found(resource_type: impl Into<String>, id: impl Into<String>) -> Self {
        Self::NotFoundDetailed {
            resource_type: resource_type.into(),
            id: id.into(),
            suggestions: Vec::new(),
        }
    }
    
    /// Create an internal error with tracking ID
    pub fn internal(message: impl Into<String>, component: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
            error_id: uuid::Uuid::new_v4().to_string(),
            component: component.into(),
        }
    }
    
    /// Create a timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            timeout: true,
            retry_count: 0,
        }
    }
    
    /// Check if this error allows retry
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AppError::Network { .. } | 
            AppError::ExternalService { .. } | 
            AppError::RateLimit { .. }
        )
    }
    
    /// Get user-friendly error message (Australian English)
    pub fn user_message(&self) -> String {
        match self {
            AppError::Database(_) | AppError::DatabaseWithContext { .. } => "We're experiencing technical difficulties. Please try again.".to_string(),
            AppError::Validation { field, message, .. } => format!("Please check your {}: {}", field, message),
            AppError::Authentication { .. } => "Please check your login details and try again.".to_string(),
            AppError::Authorisation { .. } => "You don't have permission to perform this action.".to_string(),
            AppError::NotFound => "Sorry, we couldn't find that resource.".to_string(),
            AppError::NotFoundDetailed { resource_type, .. } => format!("Sorry, we couldn't find that {}.", resource_type),
            AppError::RateLimit { retry_after_seconds, .. } => {
                format!("You're doing that too quickly. Please wait {} seconds.", retry_after_seconds)
            },
            _ => "Something went wrong. Our team has been notified.".to_string(),
        }
    }
}

// Implement From traits for common error types that are available in shared crate
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InvalidInput {
            message: format!("JSON parsing failed: {}", err),
            field: None,
            expected_format: Some("Valid JSON".to_string()),
        }
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::validation("id", format!("Invalid UUID format: {}", err))
    }
}