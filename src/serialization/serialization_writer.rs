//! The `SerializationWriter` trait for writing values to serialized payloads.

use crate::error::KiotaError;

use super::parsable::Parsable;

/// A trait for writing primitive and complex values to a serialized payload.
/// Concrete implementations exist for JSON, text, and form formats.
pub trait SerializationWriter: Send + Sync {
    /// Writes a string value.
    fn write_string_value(
        &mut self,
        key: Option<&str>,
        value: Option<&str>,
    ) -> Result<(), KiotaError>;

    /// Writes a boolean value.
    fn write_bool_value(
        &mut self,
        key: Option<&str>,
        value: Option<bool>,
    ) -> Result<(), KiotaError>;

    /// Writes an i32 value.
    fn write_i32_value(
        &mut self,
        key: Option<&str>,
        value: Option<i32>,
    ) -> Result<(), KiotaError>;

    /// Writes an i64 value.
    fn write_i64_value(
        &mut self,
        key: Option<&str>,
        value: Option<i64>,
    ) -> Result<(), KiotaError>;

    /// Writes an f32 value.
    fn write_f32_value(
        &mut self,
        key: Option<&str>,
        value: Option<f32>,
    ) -> Result<(), KiotaError>;

    /// Writes an f64 value.
    fn write_f64_value(
        &mut self,
        key: Option<&str>,
        value: Option<f64>,
    ) -> Result<(), KiotaError>;

    /// Writes a UUID value.
    fn write_uuid_value(
        &mut self,
        key: Option<&str>,
        value: Option<uuid::Uuid>,
    ) -> Result<(), KiotaError>;

    /// Writes a DateTime value.
    fn write_date_time_value(
        &mut self,
        key: Option<&str>,
        value: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), KiotaError>;

    /// Writes a date value.
    fn write_date_value(
        &mut self,
        key: Option<&str>,
        value: Option<chrono::NaiveDate>,
    ) -> Result<(), KiotaError>;

    /// Writes a time value.
    fn write_time_value(
        &mut self,
        key: Option<&str>,
        value: Option<chrono::NaiveTime>,
    ) -> Result<(), KiotaError>;

    /// Writes a byte array value.
    fn write_bytes_value(
        &mut self,
        key: Option<&str>,
        value: Option<&[u8]>,
    ) -> Result<(), KiotaError>;

    /// Writes an object value (calls `Parsable::serialize`).
    fn write_object_value(
        &mut self,
        key: Option<&str>,
        value: Option<&dyn Parsable>,
    ) -> Result<(), KiotaError>;

    /// Writes a collection of object values.
    fn write_collection_of_object_values(
        &mut self,
        key: Option<&str>,
        values: &[&dyn Parsable],
    ) -> Result<(), KiotaError>;

    /// Writes a collection of string values.
    fn write_collection_of_string_values(
        &mut self,
        key: Option<&str>,
        values: &[String],
    ) -> Result<(), KiotaError>;

    /// Writes an enum value as a string.
    fn write_enum_value(
        &mut self,
        key: Option<&str>,
        value: Option<&str>,
    ) -> Result<(), KiotaError>;

    /// Writes a null value.
    fn write_null_value(&mut self, key: Option<&str>) -> Result<(), KiotaError>;

    /// Writes additional data (untyped key-value pairs).
    fn write_additional_data(
        &mut self,
        value: &std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<(), KiotaError>;

    /// Gets the serialized content as bytes.
    fn get_serialized_content(&self) -> Result<Vec<u8>, KiotaError>;
}

/// Factory for creating `SerializationWriter` instances for a given content type.
pub trait SerializationWriterFactory: Send + Sync {
    /// Returns the content type this factory handles.
    fn get_valid_content_type(&self) -> &str;

    /// Creates a `SerializationWriter` for the given content type.
    fn get_serialization_writer(
        &self,
        content_type: &str,
    ) -> Result<Box<dyn SerializationWriter>, KiotaError>;
}
