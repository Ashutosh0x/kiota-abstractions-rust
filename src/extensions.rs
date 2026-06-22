//! Extension methods for request information and other types.

use crate::error::KiotaError;
use crate::request_information::RequestInformation;

/// Extension trait for `RequestInformation` with convenience methods.
pub trait RequestInformationExtensions {
    /// Adds query parameters from a serializable struct.
    fn add_query_parameters<Q: crate::request_option::QueryParameters>(
        &mut self,
        query_params: &Q,
    );

    /// Adds headers from a map.
    fn add_headers(&mut self, headers: &std::collections::HashMap<String, String>);

    /// Sets the base URL path parameter.
    fn set_base_url(&mut self, base_url: &str);
}

impl RequestInformationExtensions for RequestInformation {
    fn add_query_parameters<Q: crate::request_option::QueryParameters>(
        &mut self,
        query_params: &Q,
    ) {
        for (key, value) in query_params.to_query_parameters() {
            if !value.is_empty() {
                self.query_parameters.insert(key, value);
            }
        }
    }

    fn add_headers(&mut self, headers: &std::collections::HashMap<String, String>) {
        for (key, value) in headers {
            self.headers.add(key.clone(), value.clone());
        }
    }

    fn set_base_url(&mut self, base_url: &str) {
        self.path_parameters
            .insert("baseurl".to_string(), base_url.to_string());
    }
}
