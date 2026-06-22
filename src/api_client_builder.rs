//! API client builder for wiring serialization and parse node factories.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::error::KiotaError;
use crate::serialization::{ParseNodeFactory, SerializationWriterFactory};

/// A registry that maps content types to their serialization/deserialization factories.
/// Used during client initialization to register all supported formats.
#[derive(Default)]
pub struct ApiClientBuilder;

/// Global registry of parse node factories keyed by content type.
static PARSE_NODE_FACTORIES: std::sync::LazyLock<RwLock<HashMap<String, Arc<dyn ParseNodeFactory>>>> =
    std::sync::LazyLock::new(|| RwLock::new(HashMap::new()));

/// Global registry of serialization writer factories keyed by content type.
static SERIALIZATION_WRITER_FACTORIES: std::sync::LazyLock<RwLock<HashMap<String, Arc<dyn SerializationWriterFactory>>>> =
    std::sync::LazyLock::new(|| RwLock::new(HashMap::new()));

impl ApiClientBuilder {
    /// Registers a parse node factory for a given content type.
    pub fn register_default_deserializer(factory: Arc<dyn ParseNodeFactory>) {
        let content_type = factory.get_valid_content_type().to_string();
        if let Ok(mut map) = PARSE_NODE_FACTORIES.write() {
            map.insert(content_type, factory);
        }
    }

    /// Registers a serialization writer factory for a given content type.
    pub fn register_default_serializer(factory: Arc<dyn SerializationWriterFactory>) {
        let content_type = factory.get_valid_content_type().to_string();
        if let Ok(mut map) = SERIALIZATION_WRITER_FACTORIES.write() {
            map.insert(content_type, factory);
        }
    }

    /// Gets the registered parse node factory for a content type.
    pub fn get_parse_node_factory(content_type: &str) -> Result<Arc<dyn ParseNodeFactory>, KiotaError> {
        PARSE_NODE_FACTORIES
            .read()
            .map_err(|e| KiotaError::General(e.to_string()))?
            .get(content_type)
            .cloned()
            .ok_or_else(|| {
                KiotaError::SerializationError(format!(
                    "No parse node factory registered for content type: {}",
                    content_type
                ))
            })
    }

    /// Gets the registered serialization writer factory for a content type.
    pub fn get_serialization_writer_factory(
        content_type: &str,
    ) -> Result<Arc<dyn SerializationWriterFactory>, KiotaError> {
        SERIALIZATION_WRITER_FACTORIES
            .read()
            .map_err(|e| KiotaError::General(e.to_string()))?
            .get(content_type)
            .cloned()
            .ok_or_else(|| {
                KiotaError::SerializationError(format!(
                    "No serialization writer factory registered for content type: {}",
                    content_type
                ))
            })
    }
}
