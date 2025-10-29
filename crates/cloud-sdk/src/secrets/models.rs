use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    pub id: String,
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSecret {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertSecret {
    Single(CreateSecret),
    Multiple(Vec<CreateSecret>),
}

impl From<(&str, &str)> for UpsertSecret {
    fn from((name, value): (&str, &str)) -> Self {
        UpsertSecret::Single(CreateSecret {
            name: name.to_string(),
            value: value.to_string(),
        })
    }
}

impl From<&[(&str, &str)]> for UpsertSecret {
    fn from(secrets: &[(&str, &str)]) -> Self {
        UpsertSecret::Multiple(
            secrets
                .iter()
                .map(|(name, value)| CreateSecret {
                    name: name.to_string(),
                    value: value.to_string(),
                })
                .collect(),
        )
    }
}

#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpsertSecretRequest {
    #[builder(setter(into))]
    pub organization_id: String,
    #[builder(setter(into))]
    pub project_id: String,
    #[builder(setter(into))]
    pub secret: UpsertSecret,
}

impl UpsertSecretRequest {
    pub fn builder() -> UpsertSecretRequestBuilder {
        UpsertSecretRequestBuilder::default()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertSecretResponse {
    Single(Secret),
    Multiple(Vec<Secret>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretsList {
    pub items: Vec<Secret>,
    pub pagination: Pagination,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    pub total: i32,
}

#[derive(Builder, Debug)]
pub struct DeleteSecretRequest {
    #[builder(setter(into))]
    pub organization_id: String,
    #[builder(setter(into))]
    pub project_id: String,
    #[builder(setter(into))]
    pub secret_id: String,
}

impl DeleteSecretRequest {
    pub fn builder() -> DeleteSecretRequestBuilder {
        DeleteSecretRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct GetSecretRequest {
    #[builder(setter(into))]
    pub organization_id: String,
    #[builder(setter(into))]
    pub project_id: String,
    #[builder(setter(into))]
    pub secret_id: String,
}

impl GetSecretRequest {
    pub fn builder() -> GetSecretRequestBuilder {
        GetSecretRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct ListSecretsRequest {
    #[builder(setter(into))]
    pub organization_id: String,
    #[builder(setter(into))]
    pub project_id: String,
    #[builder(default, setter(strip_option))]
    pub next: Option<String>,
    #[builder(default, setter(strip_option))]
    pub prev: Option<String>,
    #[builder(default, setter(strip_option))]
    pub page_size: Option<i32>,
}

impl ListSecretsRequest {
    pub fn builder() -> ListSecretsRequestBuilder {
        ListSecretsRequestBuilder::default()
    }
}
