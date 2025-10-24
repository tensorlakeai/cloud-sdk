use super::{ContentType, Error, configuration};
use crate::applications::{apis::*, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`applications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplicationsError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_or_update_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrUpdateApplicationError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRequestError {
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationError {
    Status500(),
    UnknownValue(serde_json::Value),
}

pub async fn applications(
    configuration: &configuration::Configuration,
    namespace: &str,
    limit: Option<i32>,
    cursor: Option<&str>,
    direction: Option<models::CursorDirection>,
) -> Result<models::ApplicationsList, Error<ApplicationsError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications",
        configuration.base_path,
        namespace = urlencode(namespace)
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
                "Received `text/plain` content type response that cannot be converted to `models::ApplicationsList`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ApplicationsList`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApplicationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn create_or_update_application(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: models::Application,
    code: Vec<u8>,
) -> Result<(), Error<CreateOrUpdateApplicationError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications",
        configuration.base_path,
        namespace = urlencode(namespace)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    let mut multipart_form = reqwest::multipart::Form::new();
    multipart_form = multipart_form.text("application", application.name);
    let file_part = reqwest::multipart::Part::bytes(code).file_name("code.zip");
    multipart_form = multipart_form.part("code", file_part);
    req_builder = req_builder.multipart(multipart_form);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateOrUpdateApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn delete_application(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: &str,
) -> Result<(), Error<DeleteApplicationError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications/{application}",
        configuration.base_path,
        namespace = urlencode(namespace),
        application = urlencode(application)
    );
    let req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn delete_request(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: &str,
    request_id: &str,
) -> Result<(), Error<DeleteRequestError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications/{application}/requests/{request_id}",
        configuration.base_path,
        namespace = urlencode(namespace),
        application = urlencode(application),
        request_id = urlencode(request_id)
    );
    let req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteRequestError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_application(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: &str,
) -> Result<models::Application, Error<GetApplicationError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications/{application}",
        configuration.base_path,
        namespace = urlencode(namespace),
        application = urlencode(application)
    );
    let req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
                "Received `text/plain` content type response that cannot be converted to `models::Application`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::Application`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
