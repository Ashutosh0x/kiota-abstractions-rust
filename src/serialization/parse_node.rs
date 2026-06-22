//! The `ParseNode` trait for reading values from serialized payloads.

use crate::error::KiotaError;

use super::parsable::Parsable;

/// A trait for reading primitive and complex values from a serialized payload.
/// Concrete implementations exist for JSON, text, and form formats.
pub trait ParseNode: Send + Sync {
    /// Gets a string value from the node.
    fn get_string_value(&self) -> Result<Option<String>, KiotaError>;

    /// Gets a boolean value from the node.
    fn get_bool_value(&self) -> Result<Option<bool>, KiotaError>;

    /// Gets an i32 value from the node.
    fn get_i32_value(&self) -> Result<Option<i32>, KiotaError>;

    /// Gets an i64 value from the node.
    fn get_i64_value(&self) -> Result<Option<i64>, KiotaError>;

    /// Gets an f32 value from the node.
    fn get_f32_value(&self) -> Result<Option<f32>, KiotaError>;

    /// Gets an f64 value from the node.
    fn get_f64_value(&self) -> Result<Option<f64>, KiotaError>;

    /// Gets a UUID value from the node.
    fn get_uuid_value(&self) -> Result<Option<uuid::Uuid>, KiotaError>;

    /// Gets a DateTime value from the node.
    fn get_date_time_value(&self) -> Result<Option<chrono::DateTime<chrono::Utc>>, KiotaError>;

    /// Gets a date value from the node.
    fn get_date_value(&self) -> Result<Option<chrono::NaiveDate>, KiotaError>;

    /// Gets a time value from the node.
    fn get_time_value(&self) -> Result<Option<chrono::NaiveTime>, KiotaError>;

    /// Gets a byte array value from the node.
    fn get_bytes_value(&self) -> Result<Option<Vec<u8>>, KiotaError>;

    /// Gets a child node by key name.
    fn get_child_node(&self, key: &str) -> Result<Option<Box<dyn ParseNode>>, KiotaError>;

    /// Gets a collection of child nodes (for arrays).
    fn get_collection_of_primitive_values<T: std::str::FromStr>(
        &self,
    ) -> Result<Vec<T>, KiotaError>;

    /// Gets a collection of object values.
    fn get_collection_of_object_values(
        &self,
        factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Vec<Box<dyn Parsable>>, KiotaError>;

    /// Gets an object value using a factory function.
    fn get_object_value(
        &self,
        factory: &dyn Fn() -> Box<dyn Parsable>,
    ) -> Result<Option<Box<dyn Parsable>>, KiotaError>;

    /// Gets an enum value from the node.
    fn get_enum_value<T: std::str::FromStr>(&self) -> Result<Option<T>, KiotaError>;

    /// Gets a collection of enum values.
    fn get_collection_of_enum_values<T: std::str::FromStr>(&self) -> Result<Vec<T>, KiotaError>;
}

/// Factory for creating `ParseNode` instances from content types and byte payloads.
pub trait ParseNodeFactory: Send + Sync {
    /// Returns the content type this factory handles (e.g., "application/json").
    fn get_valid_content_type(&self) -> &str;

    /// Creates a `ParseNode` from a byte payload.
    fn get_root_parse_node(
        &self,
        content_type: &str,
        content: &[u8],
    ) -> Result<Box<dyn ParseNode>, KiotaError>;
}
