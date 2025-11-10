use data_encoding::BASE64;
use std::{collections::HashMap, io::Write};
use tensorlake_cloud_sdk::{applications::models::*, images::models::BuildStatus};

mod common;

const APP_CODE: &str = r#"
from tensorlake.applications import application, function

@application()
@function(description="A simple test function")
def simple_test_func(input_text: str) -> str:
output = helper_func(input_text)
return f"Processed: {output}"

@function()
def helper_func(value: str) -> str:
return f"Helper processed: {value}"
"#;

#[tokio::test]
async fn test_applications_operations() {
    let sdk = common::create_sdk();

    // Build test image
    let image = common::build_test_image(&sdk, "test_app", "simple_test_func").await;
    assert_eq!(BuildStatus::Succeeded, image.status);

    let (_org_id, project_id) = common::get_org_and_project_ids();
    let apps_client = sdk.applications();

    // Create zip file
    let mut zip_data = Vec::new();
    {
        let mut zip_writer = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_data));
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // Add code zip manifest
        let manifest = r#"{"functions":{"simple_test_func":{"name":"simple_test_func","module_import_name":"app"},"helper_func":{"name":"helper_func","module_import_name":"app"}}}"#;
        zip_writer
            .start_file(".tensorlake_code_manifest.json", options)
            .unwrap();
        zip_writer.write_all(manifest.as_bytes()).unwrap();

        zip_writer.start_file("app.py", options).unwrap();
        zip_writer.write_all(APP_CODE.as_bytes()).unwrap();
        zip_writer.finish().unwrap();
    }

    // Build application manifest
    let mut functions = HashMap::new();

    let function_manifest = FunctionManifest::builder()
        .name("simple_test_func".to_string())
        .description("A simple test function".to_string())
        .is_api(true)
        .initialization_timeout_sec(300)
        .timeout_sec(300)
        .resources(
            Resources::builder()
                .cpus(1.0)
                .memory_mb(1024)
                .ephemeral_disk_mb(2048)
                .build()
                .unwrap(),
        )
        .retry_policy(
            RetryPolicy::builder()
                .max_retries(0)
                .initial_delay_sec(1.0)
                .max_delay_sec(60.0)
                .delay_multiplier(2.0)
                .build()
                .unwrap(),
        )
        .parameters(vec![
            Parameter::builder()
                .name("input_text".to_string())
                .data_type("string".to_string())
                .build()
                .unwrap(),
        ])
        .return_type(serde_json::json!({"type": "string"}))
        .placement_constraints(PlacementConstraintsManifest::builder().build().unwrap())
        .max_concurrency(1)
        .build()
        .unwrap();

    let helper_function_manifest = FunctionManifest::builder()
        .name("helper_func".to_string())
        .description("".to_string())
        .is_api(false)
        .initialization_timeout_sec(300)
        .timeout_sec(300)
        .resources(
            Resources::builder()
                .cpus(1.0)
                .memory_mb(1024)
                .ephemeral_disk_mb(2048)
                .build()
                .unwrap(),
        )
        .retry_policy(
            RetryPolicy::builder()
                .max_retries(0)
                .initial_delay_sec(1.0)
                .max_delay_sec(60.0)
                .delay_multiplier(2.0)
                .build()
                .unwrap(),
        )
        .parameters(vec![
            Parameter::builder()
                .name("value".to_string())
                .data_type("string".to_string())
                .build()
                .unwrap(),
        ])
        .return_type(serde_json::json!({"type": "string"}))
        .placement_constraints(PlacementConstraintsManifest::builder().build().unwrap())
        .max_concurrency(1)
        .build()
        .unwrap();

    functions.insert("simple_test_func".to_string(), function_manifest);
    functions.insert("helper_func".to_string(), helper_function_manifest);

    let app_manifest = ApplicationManifest::builder()
        .name("test_app".to_string())
        .description("Test application".to_string())
        .tags(HashMap::new())
        .version("1.0.0".to_string())
        .functions(functions)
        .entrypoint(
            Entrypoint::builder()
                .function_name("simple_test_func".to_string())
                .input_serializer("json".to_string())
                .output_serializer("json".to_string())
                .output_type_hints_base64(BASE64.encode(r#"{"type": "string"}"#.as_bytes()))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    // Create an application
    let upsert_request = UpsertApplicationRequest::builder()
        .namespace(project_id.clone())
        .application_manifest(app_manifest)
        .code_zip(zip_data)
        .build()
        .unwrap();

    apps_client
        .upsert(&upsert_request)
        .await
        .expect("Upsert should succeed");

    // List applications
    let list_request = ListApplicationsRequest::builder()
        .namespace(project_id.clone())
        .limit(10)
        .build()
        .unwrap();

    let list_response = apps_client
        .list(&list_request)
        .await
        .expect("List should succeed");

    assert!(
        list_response
            .applications
            .iter()
            .any(|a| a.name == "test_app")
    );

    // Get application
    let get_request = GetApplicationRequest::builder()
        .namespace(project_id.clone())
        .application("test_app".to_string())
        .build()
        .unwrap();

    let get_response = apps_client
        .get(&get_request)
        .await
        .expect("Get should succeed");

    assert_eq!(get_response.name, "test_app");

    // Invoke application
    let invoke_request = InvokeApplicationRequest::builder()
        .namespace(project_id.clone())
        .application("test_app".to_string())
        .body(serde_json::json!({"input_text": "hello world"}))
        .build()
        .unwrap();

    let invoke_response = apps_client
        .invoke(&invoke_request)
        .await
        .expect("Invoke should succeed");

    let request_id = match invoke_response {
        tensorlake_cloud_sdk::applications::InvokeResponse::RequestId(id) => id,
        _ => panic!("Expected RequestId"),
    };

    assert!(!request_id.is_empty());

    // Get output for request
    // Placeholder: assume get_request method exists

    // Check output for function
    // Placeholder

    // Get output for function
    // Placeholder

    // List requests
    let list_requests_request = ListRequestsRequest::builder()
        .namespace(project_id)
        .application("test_app".to_string())
        .limit(10)
        .build()
        .unwrap();

    let _requests = apps_client
        .list_requests(&list_requests_request)
        .await
        .expect("List requests should succeed");
}
