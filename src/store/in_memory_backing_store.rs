//! In-memory implementation of `BackingStore`.

use std::any::Any;
use std::collections::{HashMap, HashSet};

use super::backing_store::BackingStore;

/// An in-memory implementation of `BackingStore` that tracks property changes
/// using a `HashMap`.
pub struct InMemoryBackingStore {
    store: HashMap<String, Box<dyn Any + Send + Sync>>,
    changed_keys: HashSet<String>,
    initialization_completed: bool,
    return_only_changed_values: bool,
}

impl InMemoryBackingStore {
    /// Creates a new `InMemoryBackingStore`.
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            changed_keys: HashSet::new(),
            initialization_completed: false,
            return_only_changed_values: false,
        }
    }
}

impl Default for InMemoryBackingStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BackingStore for InMemoryBackingStore {
    fn get(&self, key: &str) -> Option<&dyn Any> {
        if self.return_only_changed_values && !self.changed_keys.contains(key) {
            return None;
        }
        self.store.get(key).map(|v| v.as_ref() as &dyn Any)
    }

    fn set(&mut self, key: &str, value: Box<dyn Any + Send + Sync>) {
        self.store.insert(key.to_string(), value);
        if self.initialization_completed {
            self.changed_keys.insert(key.to_string());
        }
    }

    fn enumerate_keys_for_values_changed_to_null(&self) -> Vec<String> {
        self.changed_keys
            .iter()
            .filter(|key| {
                self.store
                    .get(key.as_str())
                    .map(|v| v.downcast_ref::<()>().is_some())
                    .unwrap_or(true)
            })
            .cloned()
            .collect()
    }

    fn enumerate(&self) -> Vec<(String, &dyn Any)> {
        if self.return_only_changed_values {
            self.store
                .iter()
                .filter(|(k, _)| self.changed_keys.contains(k.as_str()))
                .map(|(k, v)| (k.clone(), v.as_ref() as &dyn Any))
                .collect()
        } else {
            self.store
                .iter()
                .map(|(k, v)| (k.clone(), v.as_ref() as &dyn Any))
                .collect()
        }
    }

    fn is_initialization_completed(&self) -> bool {
        self.initialization_completed
    }

    fn set_initialization_completed(&mut self, value: bool) {
        self.initialization_completed = value;
    }

    fn return_only_changed_values(&self) -> bool {
        self.return_only_changed_values
    }

    fn set_return_only_changed_values(&mut self, value: bool) {
        self.return_only_changed_values = value;
    }

    fn clear(&mut self) {
        self.store.clear();
        self.changed_keys.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut store = InMemoryBackingStore::new();
        store.set("name", Box::new("Alice".to_string()));
        let val = store.get("name").unwrap();
        assert_eq!(val.downcast_ref::<String>().unwrap(), "Alice");
    }

    #[test]
    fn test_change_tracking() {
        let mut store = InMemoryBackingStore::new();
        store.set("initial", Box::new(1i32));
        store.set_initialization_completed(true);
        store.set("changed", Box::new(2i32));

        store.set_return_only_changed_values(true);
        assert!(store.get("initial").is_none());
        assert!(store.get("changed").is_some());
    }

    #[test]
    fn test_clear() {
        let mut store = InMemoryBackingStore::new();
        store.set("key", Box::new(42i32));
        store.clear();
        assert!(store.get("key").is_none());
    }
}
