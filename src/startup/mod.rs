//! Startup module for VMware Tanzu Secrets Manager SDK.
//! 
//! This module provides functionality for watching secrets during application startup.
//! It ensures that secrets are available before the application begins normal operation.

mod watch;

pub use watch::Watch; 