//! Base request builder that generated request builders extend.

use std::collections::HashMap;

use crate::request_adapter::RequestAdapter;

/// The base struct for all generated request builders.
/// Generated code creates request builders that contain a `BaseRequestBuilder`
/// and add typed methods for each API endpoint.
pub struct BaseRequestBuilder<'a> {
    /// The request adapter to use for sending requests.
    pub request_adapter: &'a dyn RequestAdapter,
    /// The URL template for this request builder.
    pub url_template: String,
    /// The path parameters for URL template substitution.
    pub path_parameters: HashMap<String, String>,
}

impl<'a> BaseRequestBuilder<'a> {
    /// Creates a new `BaseRequestBuilder`.
    pub fn new(
        request_adapter: &'a dyn RequestAdapter,
        url_template: impl Into<String>,
        path_parameters: HashMap<String, String>,
    ) -> Self {
        Self {
            request_adapter,
            url_template: url_template.into(),
            path_parameters,
        }
    }

    /// Creates a new `BaseRequestBuilder` from a raw URL.
    pub fn from_raw_url(
        request_adapter: &'a dyn RequestAdapter,
        raw_url: impl Into<String>,
    ) -> Self {
        let raw = raw_url.into();
        let mut path_parameters = HashMap::new();
        path_parameters.insert("request-raw-url".to_string(), raw);
        Self {
            request_adapter,
            url_template: "{+baseurl}/{+request-raw-url}".to_string(),
            path_parameters,
        }
    }
}
