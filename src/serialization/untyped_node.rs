//! Untyped node types for handling dynamic/unknown JSON structures.

use std::collections::HashMap;

/// Represents an untyped value from an API response.
/// Used when the schema doesn't define a specific type.
#[derive(Debug, Clone)]
pub enum UntypedNode {
    /// A null value.
    Null,
    /// A string value.
    String(String),
    /// A boolean value.
    Boolean(bool),
    /// An integer value.
    Integer(i64),
    /// A floating-point value.
    Float(f64),
    /// An array of untyped nodes.
    Array(Vec<UntypedNode>),
    /// An object with string keys and untyped node values.
    Object(HashMap<String, UntypedNode>),
}

impl UntypedNode {
    /// Returns true if this is a null value.
    pub fn is_null(&self) -> bool {
        matches!(self, UntypedNode::Null)
    }

    /// Attempts to get the value as a string.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            UntypedNode::String(s) => Some(s),
            _ => None,
        }
    }

    /// Attempts to get the value as a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            UntypedNode::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Attempts to get the value as an integer.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            UntypedNode::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Attempts to get the value as a float.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            UntypedNode::Float(f) => Some(*f),
            UntypedNode::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Attempts to get the value as an array.
    pub fn as_array(&self) -> Option<&[UntypedNode]> {
        match self {
            UntypedNode::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Attempts to get the value as an object.
    pub fn as_object(&self) -> Option<&HashMap<String, UntypedNode>> {
        match self {
            UntypedNode::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Gets a property from an object node.
    pub fn get(&self, key: &str) -> Option<&UntypedNode> {
        match self {
            UntypedNode::Object(obj) => obj.get(key),
            _ => None,
        }
    }
}

impl From<serde_json::Value> for UntypedNode {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => UntypedNode::Null,
            serde_json::Value::Bool(b) => UntypedNode::Boolean(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    UntypedNode::Integer(i)
                } else {
                    UntypedNode::Float(n.as_f64().unwrap_or(0.0))
                }
            }
            serde_json::Value::String(s) => UntypedNode::String(s),
            serde_json::Value::Array(arr) => {
                UntypedNode::Array(arr.into_iter().map(UntypedNode::from).collect())
            }
            serde_json::Value::Object(obj) => {
                UntypedNode::Object(
                    obj.into_iter()
                        .map(|(k, v)| (k, UntypedNode::from(v)))
                        .collect(),
                )
            }
        }
    }
}

impl From<UntypedNode> for serde_json::Value {
    fn from(node: UntypedNode) -> Self {
        match node {
            UntypedNode::Null => serde_json::Value::Null,
            UntypedNode::String(s) => serde_json::Value::String(s),
            UntypedNode::Boolean(b) => serde_json::Value::Bool(b),
            UntypedNode::Integer(i) => serde_json::Value::Number(i.into()),
            UntypedNode::Float(f) => {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            }
            UntypedNode::Array(arr) => {
                serde_json::Value::Array(arr.into_iter().map(serde_json::Value::from).collect())
            }
            UntypedNode::Object(obj) => {
                serde_json::Value::Object(
                    obj.into_iter()
                        .map(|(k, v)| (k, serde_json::Value::from(v)))
                        .collect(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json_value() {
        let json = serde_json::json!({
            "name": "Alice",
            "age": 30,
            "active": true,
            "scores": [95, 87, 92],
            "address": null
        });

        let node = UntypedNode::from(json);
        assert_eq!(node.get("name").unwrap().as_string(), Some("Alice"));
        assert_eq!(node.get("age").unwrap().as_i64(), Some(30));
        assert_eq!(node.get("active").unwrap().as_bool(), Some(true));
        assert!(node.get("address").unwrap().is_null());
    }
}
