//! Error types for Kiota SDK operations.

use std::collections::HashMap;
use std::fmt;

/// The primary error type for all Kiota SDK operations.
#[derive(Debug, thiserror::Error)]
pub enum KiotaError {
    /// An error returned by the API with a specific HTTP status code.
    #[error("API error ({status_code}): {message}")]
    ApiError {
        /// The HTTP status code returned by the API.
        status_code: u16,
        /// A human-readable error message.
        message: String,
        /// The response headers, if available.
        response_headers: Option<HashMap<String, Vec<String>>>,
    },

    /// An error that occurred during serialization or deserialization.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// An error that occurred during authentication.
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// An error that occurred during the HTTP request.
    #[error("HTTP client error: {0}")]
    HttpClientError(String),

    /// The requested resource was not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// An invalid or missing argument was provided.
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// A general/unclassified error.
    #[error("General error: {0}")]
    General(String),
}

impl KiotaError {
    /// Creates a new API error from a status code and message.
    pub fn api_error(status_code: u16, message: impl Into<String>) -> Self {
        KiotaError::ApiError {
            status_code,
            message: message.into(),
            response_headers: None,
        }
    }

    /// Creates a new API error with response headers.
    pub fn api_error_with_headers(
        status_code: u16,
        message: impl Into<String>,
        headers: HashMap<String, Vec<String>>,
    ) -> Self {
        KiotaError::ApiError {
            status_code,
            message: message.into(),
            response_headers: Some(headers),
        }
    }
}

/// A trait for API errors that can be deserialized from response bodies.
pub trait ApiException: std::error::Error + Send + Sync {
    /// The HTTP status code associated with this error.
    fn status_code(&self) -> u16;

    /// The response headers associated with this error.
    fn response_headers(&self) -> Option<&HashMap<String, Vec<String>>>;
}
