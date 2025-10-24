# Tensorlake Cloud SDK

A Rust SDK for interacting with Tensorlake Cloud APIs, providing a high-level, ergonomic interface for managing applications, functions, and requests.

## Features

- **Application Management**: Create, update, delete, and list applications
- **Request Handling**: Invoke applications and manage execution requests
- **Type Safety**: Full type safety with generated models from OpenAPI specifications
- **Async/Await**: Built on tokio for efficient asynchronous operations
- **Error Handling**: Comprehensive error types for different failure scenarios

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cloud-sdk = "0.1.0"
```

## Quick Start

```rust
use cloud_sdk::{Client, ApplicationsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your API credentials
    let client = Client::new(
        "https://api.tensorlake.ai",
        "your-api-key"
    )?;

    // Create an applications client
    let apps_client = ApplicationsClient::new(client);

    // List applications in the default namespace
    let apps = apps_client.list_applications("default").await?;
    println!("Found {} applications", apps.applications.len());

    // Get details of a specific application
    let app = apps_client.get_application("default", "my-app").await?;
    println!("Application: {} v{}", app.name, app.version);

    // Invoke an application with data
    let input_data = serde_json::json!({
        "message": "Hello, Tensorlake!"
    });
    apps_client.invoke_application("default", "my-app", Some(input_data)).await?;

    Ok(())
}
```

## Authentication

The SDK uses Bearer token authentication. Provide your API key when creating the client:

```rust
let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
```

## Available Operations

### Applications

- `list_applications(namespace)` - List all applications in a namespace
- `get_application(namespace, app_name)` - Get details of a specific application
- `create_or_update_application(namespace, app_name, data)` - Create or update an application
- `delete_application(namespace, app_name)` - Delete an application

### Requests

- `invoke_application(namespace, app_name, data)` - Invoke an application with JSON data
- `list_requests(namespace, app_name, limit, cursor, direction)` - List requests for an application
- `delete_request(namespace, app_name, request_id)` - Delete a specific request

## Error Handling

The SDK provides detailed error types for different scenarios:

```rust
use cloud_sdk::applications::apis;

match apps_client.list_applications("default").await {
    Ok(apps) => println!("Success: {:?}", apps),
    Err(apis::Error::Reqwest(e)) => eprintln!("Network error: {}", e),
    Err(apis::Error::Serde(e)) => eprintln!("Serialization error: {}", e),
    Err(apis::Error::ResponseError(content)) => {
        eprintln!("API error {}: {}", content.status, content.content)
    }
    _ => eprintln!("Other error"),
}
```

## Models

All API models are available in the `applications::models` module:

- `Application` - Application metadata and configuration
- `ApplicationRequests` - List of requests with pagination
- `CreateNamespace` - Data for creating/updating applications
- And many more...

## Testing

The SDK includes comprehensive tests using httpmock for mocking HTTP interactions. Run tests with:

```bash
cargo test
```

## License

This SDK is licensed under the Apache 2.0 License.
