//! Request information for building HTTP requests.

use std::collections::HashMap;

use crate::http_method::HttpMethod;
use crate::error::KiotaError;

/// Represents all the information needed to make an HTTP request.
/// Generated request builders populate this struct and pass it to
/// a [`RequestAdapter`](crate::RequestAdapter) for execution.
#[derive(Debug)]
pub struct RequestInformation {
    /// The HTTP method for the request.
    pub http_method: HttpMethod,
    /// The URL template for the request.
    pub url_template: String,
    /// The path parameters to substitute in the URL template.
    pub path_parameters: HashMap<String, String>,
    /// The query parameters for the request.
    pub query_parameters: HashMap<String, String>,
    /// The headers for the request.
    pub headers: RequestHeaders,
    /// The request body content.
    pub content: Option<Vec<u8>>,
    /// The content type of the request body.
    pub content_type: Option<String>,
}

impl Clone for RequestInformation {
    fn clone(&self) -> Self {
        Self {
            http_method: self.http_method.clone(),
            url_template: self.url_template.clone(),
            path_parameters: self.path_parameters.clone(),
            query_parameters: self.query_parameters.clone(),
            headers: self.headers.clone(),
            content: self.content.clone(),
            content_type: self.content_type.clone(),
        }
    }
}

/// A collection of HTTP headers that supports multiple values per key.
#[derive(Debug, Clone, Default)]
pub struct RequestHeaders {
    headers: HashMap<String, Vec<String>>,
}

impl RequestHeaders {
    /// Creates a new empty `RequestHeaders`.
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    /// Adds a header value. If the header already exists, the value is appended.
    pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers
            .entry(key.into())
            .or_insert_with(Vec::new)
            .push(value.into());
    }

    /// Sets a header value, replacing any existing values.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(key.into(), vec![value.into()]);
    }

    /// Gets all values for a header.
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.headers.get(key)
    }

    /// Gets the first value for a header.
    pub fn get_first(&self, key: &str) -> Option<&String> {
        self.headers.get(key).and_then(|v| v.first())
    }

    /// Removes a header.
    pub fn remove(&mut self, key: &str) -> Option<Vec<String>> {
        self.headers.remove(key)
    }

    /// Returns true if the header exists.
    pub fn contains(&self, key: &str) -> bool {
        self.headers.contains_key(key)
    }

    /// Returns an iterator over all headers.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.headers.iter()
    }

    /// Returns the number of distinct header names.
    pub fn len(&self) -> usize {
        self.headers.len()
    }

    /// Returns true if there are no headers.
    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }
}

impl RequestInformation {
    /// Creates a new `RequestInformation` with the given method and URL template.
    pub fn new(method: HttpMethod, url_template: impl Into<String>) -> Self {
        Self {
            http_method: method,
            url_template: url_template.into(),
            path_parameters: HashMap::new(),
            query_parameters: HashMap::new(),
            headers: RequestHeaders::new(),
            content: None,
            content_type: None,
        }
    }

    /// Sets the request body from a `Parsable` object using a `SerializationWriter`.
    pub fn set_content_from_parsable<T: crate::serialization::Parsable>(
        &mut self,
        writer_factory: &dyn crate::serialization::SerializationWriterFactory,
        content_type: &str,
        value: &T,
    ) -> Result<(), KiotaError> {
        let mut writer = writer_factory.get_serialization_writer(content_type)?;
        value.serialize(writer.as_mut())?;
        self.content = Some(writer.get_serialized_content()?);
        self.content_type = Some(content_type.to_string());
        self.headers.set("Content-Type", content_type);
        Ok(())
    }

    /// Sets a path parameter.
    pub fn set_path_parameter(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.path_parameters.insert(key.into(), value.into());
    }

    /// Sets a query parameter.
    pub fn set_query_parameter(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.query_parameters.insert(key.into(), value.into());
    }

    /// Builds the final URL by substituting path and query parameters.
    pub fn get_uri(&self) -> Result<String, KiotaError> {
        let mut uri = self.url_template.clone();

        // Replace path parameters
        for (key, value) in &self.path_parameters {
            uri = uri.replace(&format!("{{{}}}", key), value);
        }

        // Append query parameters
        if !self.query_parameters.is_empty() {
            let query: Vec<String> = self
                .query_parameters
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            uri = format!("{}?{}", uri, query.join("&"));
        }

        Ok(uri)
    }
}
