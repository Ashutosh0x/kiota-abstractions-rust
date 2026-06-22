//! The `RequestAdapter` trait — the main entry point for sending HTTP requests.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::error::KiotaError;
use crate::request_information::RequestInformation;
use crate::serialization::{Parsable, ParsableFactory, ParseNodeFactory, SerializationWriterFactory};

/// The service responsible for translating abstract `RequestInformation`
/// into native HTTP requests and processing the responses.
///
/// This is the main interface that generated API clients use to make requests.
/// Concrete implementations (e.g., using `reqwest`) handle the actual HTTP
/// communication.
#[async_trait]
pub trait RequestAdapter: Send + Sync {
    /// Sends a request that returns a deserialized object.
    async fn send<T: Parsable + Default>(
        &self,
        request_info: &RequestInformation,
        error_mappings: &HashMap<String, ParsableFactory>,
    ) -> Result<Option<T>, KiotaError>;

    /// Sends a request that returns a collection of deserialized objects.
    async fn send_collection<T: Parsable + Default>(
        &self,
        request_info: &RequestInformation,
        error_mappings: &HashMap<String, ParsableFactory>,
    ) -> Result<Vec<T>, KiotaError>;

    /// Sends a request that returns a primitive value.
    async fn send_primitive<T: std::str::FromStr + Send>(
        &self,
        request_info: &RequestInformation,
        error_mappings: &HashMap<String, ParsableFactory>,
    ) -> Result<Option<T>, KiotaError>;

    /// Sends a request that returns no content (e.g., DELETE, 204 responses).
    async fn send_no_content(
        &self,
        request_info: &RequestInformation,
        error_mappings: &HashMap<String, ParsableFactory>,
    ) -> Result<(), KiotaError>;

    /// Sends a request that returns a raw byte stream.
    async fn send_stream(
        &self,
        request_info: &RequestInformation,
        error_mappings: &HashMap<String, ParsableFactory>,
    ) -> Result<Option<Vec<u8>>, KiotaError>;

    /// Gets the serialization writer factory used by this adapter.
    fn get_serialization_writer_factory(&self) -> &dyn SerializationWriterFactory;

    /// Gets the base URL for all requests made by this adapter.
    fn get_base_url(&self) -> &str;

    /// Sets the base URL for all requests made by this adapter.
    fn set_base_url(&mut self, base_url: &str);
}
