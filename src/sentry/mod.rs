//! Sentry module for VMware Tanzu Secrets Manager SDK.
//! 
//! This module provides functionality for watching and fetching secrets using the Sentry service.
//! It includes implementations for both watching secrets in real-time and fetching them on demand.

mod watch;
mod fetch;

pub use watch::Watch;
pub use fetch::Fetch; 