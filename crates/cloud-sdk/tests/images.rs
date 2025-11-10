use tensorlake_cloud_sdk::images::models::*;

mod common;

#[tokio::test]
async fn test_images_operations() {
    let sdk = common::create_sdk();

    let image = common::build_test_image(&sdk, "test-app", "test_func").await;
    assert_eq!(BuildStatus::Succeeded, image.status);

    let build_id = image.id.clone();
    let images_client = sdk.images();

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
