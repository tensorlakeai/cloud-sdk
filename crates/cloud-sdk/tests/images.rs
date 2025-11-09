use tensorlake_cloud_sdk::images::models::*;

mod common;

#[tokio::test]
async fn test_images_operations() {
    let sdk = common::create_sdk("images");

    let images_client = sdk.images();

    // Create an image context
    let image = Image::builder()
        .name("test-image".to_string())
        .base_image("python:3.13".to_string())
        .build_operations(vec![
            ImageBuildOperation::builder()
                .operation_type(ImageBuildOperationType::RUN)
                .args(vec!["pip install requests".to_string()])
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap();

    let mut context_data = Vec::new();
    image
        .create_context_archive(&mut context_data, "0.2.75")
        .unwrap();

    // Build image
    let build_request = ImageBuildRequest::builder()
        .image_name("test-image".to_string())
        .image_tag("latest".to_string())
        .context_data(context_data)
        .application_name("test-app-image".to_string())
        .application_version("1.0.0".to_string())
        .function_name("test-func".to_string())
        .build()
        .unwrap();

    let build_result = images_client
        .build_image(build_request)
        .await
        .expect("Build should succeed");

    let build_id = build_result.id.clone();

    // List builds after the build_image operation succeeds
    let list_request = ListBuildsRequest::builder()
        .page(1)
        .page_size(10)
        .build()
        .unwrap();

    let list_response = images_client
        .list_builds(&list_request)
        .await
        .expect("List should succeed");

    assert!(list_response.items.iter().any(|b| b.public_id == build_id));

    // Get build information
    let get_request = GetBuildInfoRequest::builder()
        .build_id(build_id.clone())
        .build()
        .unwrap();

    let get_response = images_client
        .get_build_info(&get_request)
        .await
        .expect("Get should succeed");

    assert_eq!(get_response.id, build_id);
}
