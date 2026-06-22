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

/// A trait for providing access tokens for bearer authentication.
#[async_trait]
pub trait AccessTokenProvider: Send + Sync {
    /// Gets an access token for the given URL and additional claims.
    async fn get_authorization_token(
        &self,
        url: &str,
        additional_authentication_context: &std::collections::HashMap<String, String>,
    ) -> Result<String, KiotaError>;

    /// Returns the hosts that this provider is allowed to authenticate to.
    fn get_allowed_hosts_validator(&self) -> &AllowedHostsValidator;
}

/// Validates that a request URL is in the allowed hosts list.
#[derive(Debug, Clone)]
pub struct AllowedHostsValidator {
    allowed_hosts: std::collections::HashSet<String>,
}

impl AllowedHostsValidator {
    /// Creates a new validator with the given allowed hosts.
    pub fn new(hosts: Vec<String>) -> Self {
        Self {
            allowed_hosts: hosts.into_iter().map(|h| h.to_lowercase()).collect(),
        }
    }

    /// Returns true if the URL's host is in the allowed list.
    /// An empty allowed list means all hosts are allowed.
    pub fn is_url_host_valid(&self, url: &str) -> bool {
        if self.allowed_hosts.is_empty() {
            return true;
        }
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                return self.allowed_hosts.contains(&host.to_lowercase());
            }
        }
        false
    }

    /// Gets the set of allowed hosts.
    pub fn get_allowed_hosts(&self) -> &std::collections::HashSet<String> {
        &self.allowed_hosts
    }

    /// Sets the allowed hosts.
    pub fn set_allowed_hosts(&mut self, hosts: Vec<String>) {
        self.allowed_hosts = hosts.into_iter().map(|h| h.to_lowercase()).collect();
    }
}

impl Default for AllowedHostsValidator {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

/// A bearer token authentication provider that uses an `AccessTokenProvider`
/// to get tokens and adds them as `Authorization: Bearer <token>` headers.
pub struct BaseBearerTokenAuthenticationProvider<T: AccessTokenProvider> {
    /// The access token provider.
    pub access_token_provider: T,
}

impl<T: AccessTokenProvider> BaseBearerTokenAuthenticationProvider<T> {
    /// Creates a new `BaseBearerTokenAuthenticationProvider`.
    pub fn new(access_token_provider: T) -> Self {
        Self {
            access_token_provider,
        }
    }
}

#[async_trait]
impl<T: AccessTokenProvider> AuthenticationProvider for BaseBearerTokenAuthenticationProvider<T> {
    async fn authenticate_request(
        &self,
        request: &mut RequestInformation,
    ) -> Result<(), KiotaError> {
        let uri = request.get_uri()?;

        if !self
            .access_token_provider
            .get_allowed_hosts_validator()
            .is_url_host_valid(&uri)
        {
            return Ok(());
        }

        let token = self
            .access_token_provider
            .get_authorization_token(&uri, &std::collections::HashMap::new())
            .await?;

        if !token.is_empty() {
            request
                .headers
                .set("Authorization", format!("Bearer {}", token));
        }

        Ok(())
    }
}
