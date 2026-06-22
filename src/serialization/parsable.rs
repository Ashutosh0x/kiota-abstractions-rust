//! The `Parsable` trait for types that can be serialized/deserialized by Kiota.

use std::collections::HashMap;

use crate::error::KiotaError;

use super::parse_node::ParseNode;
use super::serialization_writer::SerializationWriter;

/// A trait for types that can be serialized and deserialized.
/// All Kiota-generated model types implement this trait.
pub trait Parsable: Send + Sync {
    /// Returns a map of field names to deserialization functions.
    /// Each function takes a `ParseNode` and sets the corresponding field
    /// on the implementing type.
    fn get_field_deserializers(
        &self,
    ) -> HashMap<String, Box<dyn FnMut(&mut dyn ParseNode) -> Result<(), KiotaError> + '_>>;

    /// Serializes the object to a `SerializationWriter`.
    fn serialize(&self, writer: &mut dyn SerializationWriter) -> Result<(), KiotaError>;
}

/// A factory function that creates a new instance of a `Parsable` type.
pub type ParsableFactory = Box<dyn Fn() -> Box<dyn Parsable> + Send + Sync>;
