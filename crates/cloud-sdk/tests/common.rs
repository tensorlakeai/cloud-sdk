use reqwest_vcr::{VCRMiddleware, VCRMode};
use std::{env, path::PathBuf};
use tensorlake_cloud_sdk::Sdk;

pub fn create_sdk(module: &str) -> Sdk {
    let url = env::var("TENSORLAKE_API_URL").expect("TENSORLAKE_API_URL must be set");
    let token = env::var("TENSORLAKE_API_TOKEN").expect("TENSORLAKE_API_TOKEN must be set");

    // Create SDK without middleware
    let mut sdk = Sdk::new(&url, &token).expect("Failed to create SDK");

    if env::var("TENSORLAKE_VCR_ENABLED").is_ok() {
        let cassette_path = format!("tests/vcr/{}", module);
        let middleware = VCRMiddleware::try_from(PathBuf::from(cassette_path))
            .expect("Failed to create VCR middleware");

        let mode = env::var("TENSORLAKE_VCR_REPLAY")
            .map(|value| match value.as_str() {
                "true" => VCRMode::Replay,
                _ => VCRMode::Record,
            })
            .unwrap_or(VCRMode::Record);

        let middleware = middleware.with_mode(mode);

        sdk = sdk
            .with_middleware(middleware)
            .expect("Failed to add VCR middleware to SDK");
    }

    sdk
}

pub fn get_org_and_project_ids() -> (String, String) {
    let org_id =
        env::var("TENSORLAKE_ORGANIZATION_ID").expect("TENSORLAKE_ORGANIZATION_ID must be set");
    let project_id = env::var("TENSORLAKE_PROJECT_ID").expect("TENSORLAKE_PROJECT_ID must be set");

    (org_id, project_id)
}
