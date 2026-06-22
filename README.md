# kiota-abstractions-rust

Core abstractions for [Kiota](https://github.com/microsoft/kiota)-generated Rust SDKs.

## Overview

This crate provides the foundational traits and types that all Kiota-generated Rust API clients depend on. It defines abstract interfaces for HTTP requests, serialization, and authentication — allowing different implementations to be plugged in.

## Key Traits

| Trait | Purpose |
|-------|---------|
| `RequestAdapter` | Main entry point for sending HTTP requests |
| `Parsable` | Types that can be serialized/deserialized |
| `ParseNode` | Reads values from a serialized payload |
| `SerializationWriter` | Writes values to a serialized payload |
| `AuthenticationProvider` | Provides authentication for requests |

## Companion Crates

| Crate | Purpose | Status |
|-------|---------|--------|
| `kiota-http-reqwest` | reqwest-based HTTP client | 🚧 Planned |
| `kiota-serialization-json` | JSON serialization via serde_json | 🚧 Planned |
| `kiota-serialization-text` | Text serialization | 🚧 Planned |
| `kiota-serialization-form` | Form URL-encoded serialization | 🚧 Planned |

## Usage

```rust
use kiota_abstractions::{RequestAdapter, RequestInformation, HttpMethod};

// Generated client code uses these abstractions:
let request = RequestInformation::new(HttpMethod::Get, "https://api.example.com/users/{id}");
```

## Related

- [Kiota](https://github.com/microsoft/kiota) — OpenAPI-based SDK generator
- [Kiota Rust PR #7571](https://github.com/microsoft/kiota/pull/7571) — Experimental Rust generator
- [kiota-community](https://github.com/kiota-community) — Community-driven Kiota extensions

## License

MIT
