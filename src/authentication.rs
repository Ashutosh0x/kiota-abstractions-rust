//! Authentication provider abstractions.

use async_trait::async_trait;

use crate::error::KiotaError;
use crate::request_information::RequestInformation;

/// A trait for providing authentication to outgoing requests.
/// Implementations can add bearer tokens, API keys, or other
/// authentication mechanisms to the request.
#[async_trait]
pub trait AuthenticationProvider: Send + Sync {
    /// Authenticates the given request by adding authentication
    /// information (e.g., Authorization header, API key).
    async fn authenticate_request(
        &self,
        request: &mut RequestInformation,
    ) -> Result<(), KiotaError>;
}

/// An authentication provider that does not add any authentication.
/// Useful for public APIs that don't require authentication.
#[derive(Debug, Clone, Default)]
pub struct AnonymousAuthenticationProvider;

#[async_trait]
impl AuthenticationProvider for AnonymousAuthenticationProvider {
    async fn authenticate_request(
        &self,
        _request: &mut RequestInformation,
    ) -> Result<(), KiotaError> {
        // No authentication needed
        Ok(())
    }
}

/// An authentication provider that uses an API key.
#[derive(Debug, Clone)]
pub struct ApiKeyAuthenticationProvider {
    /// The API key value.
    pub api_key: String,
    /// The header name for the API key (e.g., "X-API-Key").
    pub header_name: String,
}

impl ApiKeyAuthenticationProvider {
    /// Creates a new `ApiKeyAuthenticationProvider`.
    pub fn new(api_key: impl Into<String>, header_name: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            header_name: header_name.into(),
        }
    }
}

#[async_trait]
impl AuthenticationProvider for ApiKeyAuthenticationProvider {
    async fn authenticate_request(
        &self,
        request: &mut RequestInformation,
    ) -> Result<(), KiotaError> {
        request
            .headers
            .set(self.header_name.clone(), self.api_key.clone());
        Ok(())
    }
}
