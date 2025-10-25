use cloud_sdk::{Client, images::ImagesClient};
use httpmock::prelude::*;

#[tokio::test]
async fn test_simple_get() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET).path("/test");
        then.status(200).body("ok");
    });

    let client = Client::new(&server.base_url(), "test-token").unwrap();

    let response = client
        .http_client()
        .get(format!("{}/test", server.base_url()))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    mock.assert();
}

#[tokio::test]
async fn test_images_client_creation() {
    let client = Client::new("http://example.com", "test-token").unwrap();
    let _images_client = ImagesClient::new(client);

    // Just verify the client was created successfully
    // The actual API calls would be tested in integration tests
    // Since fields are private, we just ensure no panic occurs
}
