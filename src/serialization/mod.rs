//! Serialization abstractions for Kiota SDKs.
//!
//! This module defines the core traits for serializing and deserializing
//! API request/response bodies. Concrete implementations (JSON, text, form)
//! are provided by separate crates.

mod parsable;
mod parse_node;
mod serialization_writer;
mod untyped_node;

pub use parsable::{Parsable, ParsableFactory};
pub use parse_node::{ParseNode, ParseNodeFactory};
pub use serialization_writer::{SerializationWriter, SerializationWriterFactory};
pub use untyped_node::UntypedNode;
