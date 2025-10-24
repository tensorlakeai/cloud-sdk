use super::{ContentType, Error, configuration};
use crate::applications::{apis::*, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`invoke_application_with_object_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InvokeApplicationWithObjectV1Error {
    Status400(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_requests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRequestsError {
    Status500(),
    UnknownValue(serde_json::Value),
}

pub async fn invoke_application_with_object_v1(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: &str,
    body: Option<serde_json::Value>,
) -> Result<(), Error<InvokeApplicationWithObjectV1Error>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications/{application}",
        configuration.base_path,
        namespace = urlencode(namespace),
        application = urlencode(application)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.json(&body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<InvokeApplicationWithObjectV1Error> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn list_requests(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: &str,
    limit: Option<i32>,
    cursor: Option<&str>,
    direction: Option<models::CursorDirection>,
) -> Result<models::ApplicationRequests, Error<ListRequestsError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications/{application}/requests",
        configuration.base_path,
        namespace = urlencode(namespace),
        application = urlencode(application)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
    }
    if let Some(ref param_value) = direction {
        req_builder = req_builder.query(&[("direction", &param_value.to_string())]);
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::ApplicationRequests`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ApplicationRequests`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListRequestsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
