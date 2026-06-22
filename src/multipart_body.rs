//! Multipart request body support.

use std::collections::HashMap;

use crate::error::KiotaError;
use crate::serialization::SerializationWriter;

/// Represents a multipart request body for file uploads and mixed-content requests.
pub struct MultipartBody {
    /// The boundary string used to separate parts.
    boundary: String,
    /// The parts of the multipart body, keyed by part name.
    parts: HashMap<String, MultipartPart>,
    /// Maintains insertion order.
    part_order: Vec<String>,
}

/// A single part within a multipart body.
pub struct MultipartPart {
    /// The content type of this part (e.g., "application/json", "image/png").
    pub content_type: String,
    /// The raw content of this part.
    pub content: Vec<u8>,
    /// Optional filename for file uploads.
    pub filename: Option<String>,
}

impl MultipartBody {
    /// Creates a new `MultipartBody` with a generated boundary.
    pub fn new() -> Self {
        Self {
            boundary: format!("kiota-boundary-{}", uuid::Uuid::new_v4()),
            parts: HashMap::new(),
            part_order: Vec::new(),
        }
    }

    /// Returns the boundary string.
    pub fn boundary(&self) -> &str {
        &self.boundary
    }

    /// Returns the content type header value including the boundary.
    pub fn content_type(&self) -> String {
        format!("multipart/form-data; boundary={}", self.boundary)
    }

    /// Adds a part with string content.
    pub fn add_string_part(
        &mut self,
        name: impl Into<String>,
        content_type: impl Into<String>,
        content: impl Into<String>,
    ) {
        let name = name.into();
        self.parts.insert(
            name.clone(),
            MultipartPart {
                content_type: content_type.into(),
                content: content.into().into_bytes(),
                filename: None,
            },
        );
        self.part_order.push(name);
    }

    /// Adds a part with binary content (e.g., file upload).
    pub fn add_binary_part(
        &mut self,
        name: impl Into<String>,
        content_type: impl Into<String>,
        content: Vec<u8>,
        filename: Option<String>,
    ) {
        let name = name.into();
        self.parts.insert(
            name.clone(),
            MultipartPart {
                content_type: content_type.into(),
                content,
                filename,
            },
        );
        self.part_order.push(name);
    }

    /// Serializes the multipart body to bytes.
    pub fn serialize(&self) -> Result<Vec<u8>, KiotaError> {
        let mut output = Vec::new();

        for name in &self.part_order {
            if let Some(part) = self.parts.get(name) {
                output.extend_from_slice(format!("--{}\r\n", self.boundary).as_bytes());

                match &part.filename {
                    Some(filename) => {
                        output.extend_from_slice(
                            format!(
                                "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                                name, filename
                            )
                            .as_bytes(),
                        );
                    }
                    None => {
                        output.extend_from_slice(
                            format!("Content-Disposition: form-data; name=\"{}\"\r\n", name)
                                .as_bytes(),
                        );
                    }
                }

                output.extend_from_slice(
                    format!("Content-Type: {}\r\n\r\n", part.content_type).as_bytes(),
                );
                output.extend_from_slice(&part.content);
                output.extend_from_slice(b"\r\n");
            }
        }

        output.extend_from_slice(format!("--{}--\r\n", self.boundary).as_bytes());
        Ok(output)
    }
}

impl Default for MultipartBody {
    fn default() -> Self {
        Self::new()
    }
}
