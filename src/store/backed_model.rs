//! The `BackedModel` trait for models with a backing store.

use super::backing_store::BackingStore;

/// A trait for models that use a backing store to track property changes.
/// Generated models implement this when the backing store feature is enabled.
pub trait BackedModel: Send + Sync {
    /// Gets a reference to the model's backing store.
    fn get_backing_store(&self) -> &dyn BackingStore;

    /// Gets a mutable reference to the model's backing store.
    fn get_backing_store_mut(&mut self) -> &mut dyn BackingStore;
}
