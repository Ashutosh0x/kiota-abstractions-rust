//! Request options for middleware configuration.

/// A trait for request-level options that configure middleware behavior.
/// Implementations are stored in `RequestInformation::request_options`
/// and read by middleware handlers during request execution.
pub trait RequestOption: Send + Sync + std::any::Any {
    /// Returns a unique key identifying this option type.
    /// Middleware handlers use this key to look up their configuration.
    fn get_key(&self) -> &'static str;
}

/// Configuration for a request, combining query parameters and options.
#[derive(Debug, Default)]
pub struct RequestConfiguration<Q: QueryParameters> {
    /// The query parameters for the request.
    pub query_parameters: Option<Q>,
    /// Additional headers to add to the request.
    pub headers: Option<std::collections::HashMap<String, String>>,
    /// Request options for middleware.
    pub options: Vec<Box<dyn RequestOption>>,
}

/// A trait for types that can serialize themselves as query parameters.
pub trait QueryParameters {
    /// Converts the query parameters to a map of string key-value pairs.
    fn to_query_parameters(&self) -> std::collections::HashMap<String, String>;
}

/// Default (empty) query parameters.
#[derive(Debug, Default)]
pub struct DefaultQueryParameters;

impl QueryParameters for DefaultQueryParameters {
    fn to_query_parameters(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }
}
