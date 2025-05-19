//! Fetch implementation for the Sentry service.
//! 
//! This module provides functionality to fetch secrets on demand.

use crate::Result;
use crate::spiffe::SpiffeWorkloadApiClient;

/// A client for fetching secrets.
/// 
/// This struct provides functionality to fetch secrets from the Sentry service.
pub struct Fetch<C: SpiffeWorkloadApiClient> {
    /// The SPIFFE workload API client
    client: C,
}

impl<C: SpiffeWorkloadApiClient> Fetch<C> {
    /// Creates a new Fetch instance.
    /// 
    /// # Arguments
    /// 
    /// * `client` - A SPIFFE workload API client
    /// 
    /// # Returns
    /// 
    /// A new Fetch instance
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Fetches a secret from the Sentry service.
    /// 
    /// # Returns
    /// 
    /// A Result containing the fetched secret or an error
    pub async fn fetch(&self) -> Result<String> {
        self.client.fetch_secret().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spiffe::MockSpiffeClient;

    #[tokio::test]
    async fn test_fetch_creation() {
        let client = MockSpiffeClient::new();
        let _fetch = Fetch::new(client);
        // No need to check if client exists, it's guaranteed by the type system
    }

    #[tokio::test]
    async fn test_fetch_secret() {
        let mut client = MockSpiffeClient::new();
        client.expect_fetch_secret()
            .returning(|| Ok(String::from("test-secret")));
        
        let fetch = Fetch::new(client);
        
        let result = fetch.fetch().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-secret");
    }
} 