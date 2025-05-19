#![cfg(feature = "testing")]

use vsecm_sdk_rust::sentry::{Watch, Fetch};
use vsecm_sdk_rust::startup::Watch as StartupWatch;
use vsecm_sdk_rust::spiffe::MockSpiffeClient;
use std::time::Duration;

#[tokio::test]
async fn test_sentry_watch_and_fetch() {
    // Create a mock client
    let mut client = MockSpiffeClient::new();
    
    // Setup expectations for clone
    client.expect_clone().return_once(|| {
        let mut clone = MockSpiffeClient::new();
        clone.expect_fetch_secret()
            .returning(|| Ok(String::from("test-secret")));
        clone
    });
    
    // Setup expectations for fetch_secret
    client.expect_fetch_secret()
        .returning(|| Ok(String::from("test-secret")));
    
    // Test Fetch
    let fetch = Fetch::new(client.clone());
    let secret = fetch.fetch().await;
    assert!(secret.is_ok());
    assert_eq!(secret.unwrap(), "test-secret");
    
    // Create a new client for Watch test
    let mut client = MockSpiffeClient::new();
    client.expect_clone().return_once(|| {
        let mut clone = MockSpiffeClient::new();
        clone.expect_fetch_secret()
            .returning(|| Ok(String::from("test-secret")));
        clone
    });
    
    client.expect_fetch_secret()
        .returning(|| Ok(String::from("test-secret")));
    
    // Test Watch
    let watch = Watch::new(client, Duration::from_millis(100));
    
    // Start the watch in a separate task
    let watch_handle = tokio::spawn(async move {
        watch.watch(|secret| {
            assert_eq!(secret, "test-secret");
            Ok(())
        }).await
    });
    
    // Let it run for a bit
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    // Cancel the watch
    watch_handle.abort();
}

#[tokio::test]
async fn test_startup_watch() {
    // Create a mock client
    let mut client = MockSpiffeClient::new();
    
    // Setup expectations for clone
    client.expect_clone().return_once(|| {
        let mut clone = MockSpiffeClient::new();
        clone.expect_fetch_secret()
            .returning(|| Ok(String::from("test-secret")));
        clone
    });
    
    // Setup expectations for fetch_secret
    client.expect_fetch_secret()
        .returning(|| Ok(String::from("test-secret")));
    
    // Create a watch instance
    let watch = StartupWatch::new(client, Duration::from_millis(100));
    
    // Start the watch in a separate task
    let watch_handle = tokio::spawn(async move {
        watch.watch(|secret| {
            assert_eq!(secret, "test-secret");
            Ok(())
        }).await
    });
    
    // Let it run for a bit
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    // Cancel the watch
    watch_handle.abort();
} 