//! Watch implementation for the Sentry service.
//! 
//! This module provides functionality to watch for secret updates in real-time.

use crate::Result;
use crate::spiffe::SpiffeWorkloadApiClient;
use std::time::Duration;

/// A client for watching secrets in real-time.
/// 
/// This struct provides functionality to watch for secret updates and execute a callback
/// when changes are detected.
pub struct Watch<C: SpiffeWorkloadApiClient> {
    /// The SPIFFE workload API client
    client: C,
    /// The interval between watch checks
    interval: Duration,
}

impl<C: SpiffeWorkloadApiClient> Watch<C> {
    /// Creates a new Watch instance.
    /// 
    /// # Arguments
    /// 
    /// * `client` - A SPIFFE workload API client
    /// * `interval` - The duration between watch checks
    /// 
    /// # Returns
    /// 
    /// A new Watch instance
    pub fn new(client: C, interval: Duration) -> Self {
        Self { client, interval }
    }

    /// Starts watching for secret updates.
    /// 
    /// # Arguments
    /// 
    /// * `callback` - A closure that will be called when a secret update is detected
    /// 
    /// # Returns
    /// 
    /// A Result indicating success or failure
    pub async fn watch<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(String) -> Result<()>,
    {
        loop {
            // Fetch the secret
            let secret = self.client.fetch_secret().await?;
            
            // Call the callback with the secret
            callback(secret)?;
            
            // Sleep for the interval
            tokio::time::sleep(self.interval).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::time::Duration;
    use crate::spiffe::MockSpiffeClient;

    #[tokio::test]
    async fn test_watch_creation() {
        let mut client = MockSpiffeClient::new();
        client.expect_clone().return_once(|| MockSpiffeClient::new());
        
        let interval = Duration::from_millis(100);
        let watch = Watch::new(client, interval);
        assert_eq!(watch.interval, interval);
    }

    #[tokio::test]
    async fn test_watch_callback() {
        let mut client = MockSpiffeClient::new();
        client.expect_clone().return_once(|| {
            let mut clone = MockSpiffeClient::new();
            clone.expect_fetch_secret()
                .returning(|| Ok(String::from("test-secret")));
            clone
        });
        
        client.expect_fetch_secret()
            .returning(|| Ok(String::from("test-secret")));
        
        let interval = Duration::from_millis(100);
        let watch = Watch::new(client, interval);
        
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        // Spawn the watch in a separate task
        let watch_handle = tokio::spawn(async move {
            watch.watch(move |_secret| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }).await
        });

        // Wait for a short time to allow some callbacks
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // Cancel the watch
        watch_handle.abort();
        
        // Verify that the callback was called at least once
        assert!(counter.load(Ordering::SeqCst) > 0);
    }
} 