use std::env;
use tensorlake_cloud_sdk::{Sdk, images::models::*};

pub fn create_sdk() -> Sdk {
    let url = env::var("TENSORLAKE_API_URL").expect("TENSORLAKE_API_URL must be set");
    let token = env::var("TENSORLAKE_API_TOKEN").expect("TENSORLAKE_API_TOKEN must be set");

    Sdk::new(&url, &token).expect("Failed to create SDK")
}

#[allow(dead_code)]
pub async fn build_test_image(
    sdk: &Sdk,
    application_name: &str,
    func_name: &str,
) -> ImageBuildResult {
    let images_client = sdk.images();

    // Create an image context
    let image = Image::builder()
        .name("test-integration-image".to_string())
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

    // Build image
    let build_request = ImageBuildRequest::builder()
        .image(image)
        .image_tag("latest".to_string())
        .application_name(application_name.to_string())
        .application_version("1.0.1".to_string())
        .function_name(func_name.to_string())
        .sdk_version("0.2.75".to_string())
        .build()
        .unwrap();

    images_client.build_image(build_request).await.unwrap()
}

#[allow(dead_code)]
pub fn get_org_and_project_ids() -> (String, String) {
    let org_id =
        env::var("TENSORLAKE_ORGANIZATION_ID").expect("TENSORLAKE_ORGANIZATION_ID must be set");
    let project_id = env::var("TENSORLAKE_PROJECT_ID").expect("TENSORLAKE_PROJECT_ID must be set");

    (org_id, project_id)
}
