//! # Poll Progress Updates Example
//!
//! This example demonstrates how to poll for progress updates on a Tensorlake request.
//!
//! The example:
//! - Fetches request progress updates every second
//! - Uses JSON mode (Paginated) for response handling
//! - Tracks pagination state with `next_token`
//! - Prints updates to stdout
//! - Exits when a `RequestFinished` event is received
//!
//! ## Running the Example
//!
//! Set the following environment variables and run:
//!
//! ```sh
//! export TENSORLAKE_API_URL=<your-api-url>
//! export TENSORLAKE_API_KEY=<your-api-key>
//! export TENSORLAKE_NAMESPACE=<namespace>
//! export TENSORLAKE_APPLICATION=<application-name>
//! export TENSORLAKE_REQUEST_ID=<request-id>
//!
//! cargo run --example poll_progress_updates -p tensorlake-cloud-sdk
//! ```
//!
//! Or invoke it with environment variables inline:
//!
//! ```sh
//! TENSORLAKE_API_URL=<url> TENSORLAKE_API_KEY=<key> \
//! TENSORLAKE_NAMESPACE=<ns> TENSORLAKE_APPLICATION=<app> \
//! TENSORLAKE_REQUEST_ID=<req-id> \
//! cargo run --example poll_progress_updates -p tensorlake-cloud-sdk
//! ```

use std::env;
use std::time::Duration;
use tensorlake_cloud_sdk::Sdk;
use tensorlake_cloud_sdk::applications::models::{
    ProgressUpdatesRequest, ProgressUpdatesRequestMode,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API credentials from environment variables
    let api_url =
        env::var("TENSORLAKE_API_URL").expect("TENSORLAKE_API_URL environment variable not set");
    let api_key =
        env::var("TENSORLAKE_API_KEY").expect("TENSORLAKE_API_KEY environment variable not set");
    let namespace = env::var("TENSORLAKE_NAMESPACE")
        .expect("TENSORLAKE_NAMESPACE environment variable not set");
    let application = env::var("TENSORLAKE_APPLICATION")
        .expect("TENSORLAKE_APPLICATION environment variable not set");
    let request_id = env::var("TENSORLAKE_REQUEST_ID")
        .expect("TENSORLAKE_REQUEST_ID environment variable not set");

    // Create SDK instance
    let sdk = Sdk::new(&api_url, &api_key)?;
    let apps_client = sdk.applications();

    // Initialize next_token for pagination
    let mut next_token: Option<String> = None;

    // Poll for updates every second
    println!("==> Polling for progress updates...");
    'outer: loop {
        let request = ProgressUpdatesRequest::builder()
            .namespace(&namespace)
            .application(&application)
            .request_id(&request_id)
            .mode(ProgressUpdatesRequestMode::Paginated(next_token.clone()))
            .build()
            .unwrap();

        let response = apps_client.get_progress_updates(&request).await?;
        let progress_updates = response.json();

        // If we have updates, print them and update next_token
        if !progress_updates.updates.is_empty() {
            for update in &progress_updates.updates {
                println!("{:?}", update);
                // Check if this is a RequestFinished event
                if update.is_terminal() {
                    break 'outer;
                }
            }
            // Refresh next_token for next poll
            next_token = progress_updates.next_token.clone();
        }
        // If no updates, reuse the previous next_token (already handled above)

        // Wait 1 second before polling again
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    println!("==> Polling complete.");

    Ok(())
}
