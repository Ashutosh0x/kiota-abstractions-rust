//! Response handler abstractions.

use async_trait::async_trait;

use crate::error::KiotaError;

/// A trait for handling raw HTTP responses before deserialization.
/// Implementations can inspect, transform, or short-circuit the response
/// processing pipeline.
#[async_trait]
pub trait ResponseHandler: Send + Sync {
    /// Handles the response. Returns the native response object if the
    /// handler wants to take over response processing, or None to let
    /// the default deserialization proceed.
    async fn handle_response(
        &self,
        response: &dyn std::any::Any,
        error_mappings: &std::collections::HashMap<
            String,
            crate::serialization::ParsableFactory,
        >,
    ) -> Result<Option<Box<dyn std::any::Any + Send>>, KiotaError>;
}

/// A request option that attaches a response handler to a request.
pub struct ResponseHandlerOption {
    /// The response handler to use.
    pub response_handler: Box<dyn ResponseHandler>,
}

impl ResponseHandlerOption {
    /// The key used to store this option in request options.
    pub const KEY: &'static str = "ResponseHandlerOption";

    /// Creates a new `ResponseHandlerOption`.
    pub fn new(handler: impl ResponseHandler + 'static) -> Self {
        Self {
            response_handler: Box::new(handler),
        }
    }
}

impl crate::request_option::RequestOption for ResponseHandlerOption {
    fn get_key(&self) -> &'static str {
        Self::KEY
    }
}

impl std::fmt::Debug for ResponseHandlerOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseHandlerOption")
            .field("key", &Self::KEY)
            .finish()
    }
}
