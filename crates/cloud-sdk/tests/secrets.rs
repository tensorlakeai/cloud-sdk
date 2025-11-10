use tensorlake_cloud_sdk::secrets::models::*;

mod common;

#[tokio::test]
async fn test_secrets_operations() {
    let sdk = common::create_sdk();
    let (org_id, project_id) = common::get_org_and_project_ids();

    let secrets_client = sdk.secrets();

    // Create a new secret
    let upsert_request = UpsertSecretRequest::builder()
        .organization_id(org_id.clone())
        .project_id(project_id.clone())
        .secrets(vec![
            ("test_secret_1", "initial_value"),
            ("test_secret_2", "initial_value"),
        ])
        .build()
        .unwrap();

    secrets_client
        .upsert(upsert_request)
        .await
        .expect("Upsert should succeed");

    // List all secrets
    let list_request = ListSecretsRequest::builder()
        .organization_id(org_id.clone())
        .project_id(project_id.clone())
        .page_size(10)
        .build()
        .unwrap();

    let list_response = secrets_client
        .list(&list_request)
        .await
        .expect("List should succeed");

    assert_eq!(2, list_response.items.len());
    let secret = list_response.items.first().unwrap();
    assert_eq!("test_secret_1", secret.name);
    let secret = list_response.items.last().unwrap();
    assert_eq!("test_secret_2", secret.name);

    // Get the secret created earlier
    let get_request = GetSecretRequest::builder()
        .organization_id(org_id.clone())
        .project_id(project_id.clone())
        .secret_id(secret.id.clone())
        .build()
        .unwrap();

    let get_response = secrets_client
        .get(&get_request)
        .await
        .expect("Get should succeed");

    assert_eq!(secret.id, get_response.id);
    assert_eq!(secret.name, get_response.name);
    assert_eq!(secret.created_at, get_response.created_at);

    // Update the secret
    let update_request = UpsertSecretRequest::builder()
        .organization_id(org_id.clone())
        .project_id(project_id.clone())
        .secrets(vec![
            ("test_secret_1", "updated_value"),
            ("test_secret_2", "updated_value"),
        ])
        .build()
        .unwrap();

    secrets_client
        .upsert(update_request)
        .await
        .expect("Update should succeed");

    let list_response = secrets_client
        .list(&list_request)
        .await
        .expect("List should succeed");

    assert_eq!(2, list_response.items.len());
    let first = list_response.items.first().unwrap();
    let last = list_response.items.last().unwrap();

    // Delete secrets
    let delete_request = DeleteSecretRequest::builder()
        .organization_id(org_id.clone())
        .project_id(project_id.clone())
        .secret_id(first.id.clone())
        .build()
        .unwrap();

    secrets_client
        .delete(&delete_request)
        .await
        .expect("Delete should succeed");

    let delete_request = DeleteSecretRequest::builder()
        .organization_id(org_id.clone())
        .project_id(project_id.clone())
        .secret_id(last.id.clone())
        .build()
        .unwrap();

    secrets_client
        .delete(&delete_request)
        .await
        .expect("Delete should succeed");

    // List all secrets again
    let final_list_response = secrets_client
        .list(&list_request)
        .await
        .expect("Final list should succeed");

    assert_eq!(0, final_list_response.items.len());
}
