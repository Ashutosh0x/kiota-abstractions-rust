//! A case-insensitive map implementation for HTTP headers.

use std::collections::HashMap;
use std::fmt;

/// A map with case-insensitive string keys.
/// Used primarily for HTTP headers where header names are case-insensitive
/// per RFC 7230.
#[derive(Debug, Clone, Default)]
pub struct CaseInsensitiveMap<V> {
    inner: HashMap<String, V>,
}

impl<V> CaseInsensitiveMap<V> {
    /// Creates a new empty `CaseInsensitiveMap`.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Inserts a key-value pair. The key is stored in lowercase.
    pub fn insert(&mut self, key: impl Into<String>, value: V) -> Option<V> {
        self.inner.insert(key.into().to_lowercase(), value)
    }

    /// Gets a reference to the value for the given key (case-insensitive).
    pub fn get(&self, key: &str) -> Option<&V> {
        self.inner.get(&key.to_lowercase())
    }

    /// Gets a mutable reference to the value for the given key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        self.inner.get_mut(&key.to_lowercase())
    }

    /// Removes a key-value pair (case-insensitive).
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.inner.remove(&key.to_lowercase())
    }

    /// Returns true if the map contains the key (case-insensitive).
    pub fn contains_key(&self, key: &str) -> bool {
        self.inner.contains_key(&key.to_lowercase())
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator over the key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.inner.iter()
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<V: fmt::Display> fmt::Display for CaseInsensitiveMap<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (k, v)) in self.inner.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_insensitive_insert_and_get() {
        let mut map = CaseInsensitiveMap::new();
        map.insert("Content-Type", "application/json");
        assert_eq!(map.get("content-type"), Some(&"application/json"));
        assert_eq!(map.get("CONTENT-TYPE"), Some(&"application/json"));
        assert_eq!(map.get("Content-Type"), Some(&"application/json"));
    }

    #[test]
    fn test_case_insensitive_overwrite() {
        let mut map = CaseInsensitiveMap::new();
        map.insert("Accept", "text/html");
        map.insert("ACCEPT", "application/json");
        assert_eq!(map.get("accept"), Some(&"application/json"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_case_insensitive_remove() {
        let mut map = CaseInsensitiveMap::new();
        map.insert("Authorization", "Bearer token");
        assert!(map.contains_key("authorization"));
        map.remove("AUTHORIZATION");
        assert!(!map.contains_key("Authorization"));
    }
}
