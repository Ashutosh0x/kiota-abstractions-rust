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

pub mod additional_data;
pub mod api_client_builder;
pub mod authentication;
pub mod base_request_builder;
pub mod case_insensitive_map;
pub mod date_only;
pub mod error;
pub mod extensions;
pub mod http_method;
pub mod multipart_body;
pub mod native_response_handler;
pub mod request_adapter;
pub mod request_information;
pub mod request_option;
pub mod response_handler;
pub mod serialization;
pub mod store;
pub mod time_only;

// Re-export key types at crate root
pub use additional_data::AdditionalDataHolder;
pub use api_client_builder::ApiClientBuilder;
pub use authentication::{
    AccessTokenProvider, AllowedHostsValidator, AuthenticationProvider,
    BaseBearerTokenAuthenticationProvider,
};
pub use base_request_builder::BaseRequestBuilder;
pub use case_insensitive_map::CaseInsensitiveMap;
pub use date_only::DateOnly;
pub use error::KiotaError;
pub use extensions::RequestInformationExtensions;
pub use http_method::HttpMethod;
pub use multipart_body::MultipartBody;
pub use request_adapter::RequestAdapter;
pub use request_information::RequestInformation;
pub use request_option::{QueryParameters, RequestOption};
pub use response_handler::ResponseHandler;
pub use serialization::{Parsable, ParsableFactory, ParseNode, ParseNodeFactory};
pub use serialization::{SerializationWriter, SerializationWriterFactory, UntypedNode};
pub use store::{BackedModel, BackingStore, InMemoryBackingStore};
pub use time_only::TimeOnly;
