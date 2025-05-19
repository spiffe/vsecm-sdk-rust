//! Error types for the VMware Tanzu Secrets Manager SDK.
//! 
//! This module defines the error types that can occur when using the SDK.

use thiserror::Error;

/// Represents all possible errors that can occur when using the SDK.
#[derive(Error, Debug)]
pub enum Error {
    /// Errors related to SPIFFE operations
    #[error("SPIFFE error: {0}")]
    Spiffe(#[from] anyhow::Error),

    /// Errors related to HTTP operations
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Errors related to JSON serialization/deserialization
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Errors related to invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// Errors related to watch operations
    #[error("Watch error: {0}")]
    Watch(String),

    /// Errors related to fetch operations
    #[error("Fetch error: {0}")]
    Fetch(String),
} 