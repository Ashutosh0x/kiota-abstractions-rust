//! Trait for models that hold additional/extra properties.

use std::collections::HashMap;

/// A trait for models that can hold additional properties not defined
/// in the schema. These "extra" properties are preserved during
/// deserialization and re-emitted during serialization.
pub trait AdditionalDataHolder {
    /// Gets the additional data stored on this model.
    fn get_additional_data(&self) -> &HashMap<String, serde_json::Value>;

    /// Gets a mutable reference to the additional data.
    fn get_additional_data_mut(&mut self) -> &mut HashMap<String, serde_json::Value>;

    /// Sets the additional data on this model.
    fn set_additional_data(&mut self, data: HashMap<String, serde_json::Value>);
}
