//! # VMware Tanzu Secrets Manager SDK for Rust
//! 
//! This crate provides a Rust implementation of the VMware Tanzu Secrets Manager SDK.
//! It allows applications to securely fetch and watch secrets using SPIFFE for authentication.
//! 
//! ## Features
//! 
//! - Secure secret fetching using SPIFFE authentication
//! - Watch functionality for real-time secret updates
//! - Startup watch for initial secret loading
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use vsecm_sdk_rust::sentry::{Watch, Fetch};
//! use vsecm_sdk_rust::startup::Watch as StartupWatch;
//! use vsecm_sdk_rust::spiffe::SpiffeWorkloadApiClient;
//! use spiffe::workload_api::client::WorkloadApiClient;
//! use std::time::Duration;
//! use anyhow::Result;
//! 
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Create a SPIFFE client
//!     let client = WorkloadApiClient::new_from_path("unix:///tmp/spire-agent/public/api.sock").await?;
//!     
//!     // Create a watch instance
//!     let watch = Watch::new(client.clone(), Duration::from_secs(5));
//!     
//!     // Start watching
//!     watch.watch(|secret| {
//!         println!("Received secret: {}", secret);
//!         Ok(())
//!     }).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod sentry;
pub mod startup;
pub mod error;
pub mod spiffe;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>; 