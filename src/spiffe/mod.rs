//! SPIFFE client interface for the SDK.
//! 
//! This module provides an interface for interacting with the SPIFFE Workload API.

use async_trait::async_trait;
use crate::Result;

/// A trait that defines the interface for a SPIFFE Workload API client.
/// 
/// This trait is used to abstract the WorkloadApiClient from the spiffe crate,
/// making it easier to mock for testing.
#[async_trait]
pub trait SpiffeWorkloadApiClient: Clone + Send + Sync {
    /// Fetches a secret from the workload API.
    async fn fetch_secret(&self) -> Result<String>;
}

// Implementation for the spiffe crate's WorkloadApiClient
#[cfg(not(test))]
#[async_trait]
impl SpiffeWorkloadApiClient for spiffe::workload_api::client::WorkloadApiClient {
    async fn fetch_secret(&self) -> Result<String> {
        // In a real implementation, this would interact with the SPIFFE Workload API
        // For now, we just return an empty string
        Ok(String::new())
    }
}

// Make mockall available for both unit tests and integration tests
#[cfg(any(test, feature = "testing"))]
pub use mockall;

// Create a mock of the SpiffeWorkloadApiClient for testing
#[cfg(any(test, feature = "testing"))]
mockall::mock! {
    pub SpiffeClient {}
    
    #[async_trait]
    impl SpiffeWorkloadApiClient for SpiffeClient {
        async fn fetch_secret(&self) -> Result<String>;
    }
    
    impl Clone for SpiffeClient {
        fn clone(&self) -> Self;
    }
} 