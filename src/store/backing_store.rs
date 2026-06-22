//! The `BackingStore` trait for property change tracking.

use std::any::Any;

/// A store that tracks property changes on a model.
/// When enabled, only modified properties are included in serialization,
/// enabling efficient PATCH requests.
pub trait BackingStore: Send + Sync {
    /// Gets a value from the store by key.
    fn get(&self, key: &str) -> Option<&dyn Any>;

    /// Sets a value in the store, tracking the change.
    fn set(&mut self, key: &str, value: Box<dyn Any + Send + Sync>);

    /// Returns all keys that have been modified since the last clear.
    fn enumerate_keys_for_values_changed_to_null(&self) -> Vec<String>;

    /// Returns an iterator over all stored key-value pairs.
    fn enumerate(&self) -> Vec<(String, &dyn Any)>;

    /// Returns whether initialization is completed.
    /// When false, changes are not tracked (used during deserialization).
    fn is_initialization_completed(&self) -> bool;

    /// Sets whether initialization is completed.
    fn set_initialization_completed(&mut self, value: bool);

    /// Returns whether the return-only-changed-values mode is enabled.
    fn return_only_changed_values(&self) -> bool;

    /// Sets the return-only-changed-values mode.
    fn set_return_only_changed_values(&mut self, value: bool);

    /// Clears all tracked changes.
    fn clear(&mut self);
}
