//! # Kiota Abstractions for Rust
//!
//! Core abstractions for Kiota-generated Rust SDKs. This crate provides the
//! foundational traits and types that all Kiota-generated API clients depend on.
//!
//! ## Overview
//!
//! Kiota is an OpenAPI-based SDK generator that produces strongly-typed API
//! clients. This crate defines the abstract interfaces that generated code
//! references, allowing different implementations (HTTP clients, serialization
//! formats, authentication providers) to be plugged in.
//!
//! ## Key Traits
//!
//! - [`RequestAdapter`] — The main entry point for sending HTTP requests
//! - [`Parsable`] — Types that can be serialized/deserialized
//! - [`ParseNode`] — Reads values from a serialized payload
//! - [`SerializationWriter`] — Writes values to a serialized payload
//! - [`AuthenticationProvider`] — Provides authentication for requests
//!
//! ## Example
//!
//! ```rust,no_run
//! use kiota_abstractions::RequestAdapter;
//! // Generated client code uses RequestAdapter to make API calls
//! ```

pub mod authentication;
pub mod case_insensitive_map;
pub mod date_only;
pub mod error;
pub mod http_method;
pub mod request_adapter;
pub mod request_information;
pub mod request_option;
pub mod response_handler;
pub mod serialization;
pub mod time_only;

// Re-export key types at crate root
pub use authentication::AuthenticationProvider;
pub use case_insensitive_map::CaseInsensitiveMap;
pub use date_only::DateOnly;
pub use error::KiotaError;
pub use http_method::HttpMethod;
pub use request_adapter::RequestAdapter;
pub use request_information::RequestInformation;
pub use request_option::{QueryParameters, RequestOption};
pub use response_handler::ResponseHandler;
pub use serialization::{Parsable, ParsableFactory, ParseNode, ParseNodeFactory};
pub use serialization::{SerializationWriter, SerializationWriterFactory};
pub use time_only::TimeOnly;
