// Hanzo KMS
// (c) 2025-2026 Hanzo AI, Inc., under BSD-3-Clause license

//! Official Rust SDK for Hanzo KMS

pub mod auth;
pub mod client;
pub mod error;
pub mod resources;

#[cfg(test)]
mod tests;

pub mod secrets {
    pub use crate::resources::secrets::*;
}

pub mod kms {
    pub use crate::resources::kms::*;
}

pub use auth::AuthMethod;
pub use client::Client;
pub use error::KmsError;
pub use resources::kms::{decode_base64, encode_base64};
