//! Backing store for tracking property changes.
//!
//! The backing store pattern allows models to track which properties have
//! been modified, enabling efficient PATCH operations that only send
//! changed fields.

mod backing_store;
mod backed_model;
mod in_memory_backing_store;

pub use backing_store::BackingStore;
pub use backed_model::BackedModel;
pub use in_memory_backing_store::InMemoryBackingStore;
