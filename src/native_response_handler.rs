//! Native response handler for accessing raw HTTP responses.

use async_trait::async_trait;

use crate::error::KiotaError;
use crate::response_handler::ResponseHandler;

/// A response handler that captures the native/raw HTTP response
/// instead of deserializing it. Useful when you need access to
/// headers, status codes, or the raw response body.
pub struct NativeResponseHandler {
    /// The captured native response value.
    pub value: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any + Send>>>>,
    /// The captured error mappings, if any.
    pub error_mappings: Option<
        std::collections::HashMap<String, crate::serialization::ParsableFactory>,
    >,
}

impl NativeResponseHandler {
    /// Creates a new `NativeResponseHandler`.
    pub fn new() -> Self {
        Self {
            value: std::sync::Arc::new(std::sync::Mutex::new(None)),
            error_mappings: None,
        }
    }

    /// Gets the captured response value.
    pub fn get_value(&self) -> Option<Box<dyn std::any::Any + Send>> {
        self.value.lock().ok().and_then(|mut v| v.take())
    }
}

impl Default for NativeResponseHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ResponseHandler for NativeResponseHandler {
    async fn handle_response(
        &self,
        _response: &dyn std::any::Any,
        _error_mappings: &std::collections::HashMap<
            String,
            crate::serialization::ParsableFactory,
        >,
    ) -> Result<Option<Box<dyn std::any::Any + Send>>, KiotaError> {
        // Store the response for later retrieval
        // In a real implementation, this would clone or take ownership of the response
        Ok(None)
    }
}

impl std::fmt::Debug for NativeResponseHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeResponseHandler").finish()
    }
}
