use super::{ContentType, Error, configuration};
use crate::applications::{apis::*, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`find_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindRequestError {
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_download_fn_output_payload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1DownloadFnOutputPayloadError {
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_download_fn_output_payload_head`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1DownloadFnOutputPayloadHeadError {
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_download_fn_output_payload_simple`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1DownloadFnOutputPayloadSimpleError {
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

pub async fn find_request(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: &str,
    request_id: &str,
) -> Result<models::Request, Error<FindRequestError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications/{application}/requests/{request_id}",
        configuration.base_path,
        namespace = urlencode(namespace),
        application = urlencode(application),
        request_id = urlencode(request_id)
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
                "Received `text/plain` content type response that cannot be converted to `models::Request`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::Request`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<FindRequestError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn v1_download_fn_output_payload(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: &str,
    request_id: &str,
    fn_call_id: &str,
) -> Result<(), Error<V1DownloadFnOutputPayloadError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications/{application}/requests/{request_id}/output/{fn_call_id}",
        configuration.base_path,
        namespace = urlencode(namespace),
        application = urlencode(application),
        request_id = urlencode(request_id),
        fn_call_id = urlencode(fn_call_id)
    );
    let req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<V1DownloadFnOutputPayloadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn v1_download_fn_output_payload_head(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: &str,
    request_id: &str,
) -> Result<(), Error<V1DownloadFnOutputPayloadHeadError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications/{application}/requests/{request_id}/output",
        configuration.base_path,
        namespace = urlencode(namespace),
        application = urlencode(application),
        request_id = urlencode(request_id)
    );
    let req_builder = configuration
        .client
        .request(reqwest::Method::HEAD, &uri_str);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<V1DownloadFnOutputPayloadHeadError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn v1_download_fn_output_payload_simple(
    configuration: &configuration::Configuration,
    namespace: &str,
    application: &str,
    request_id: &str,
) -> Result<(), Error<V1DownloadFnOutputPayloadSimpleError>> {
    let uri_str = format!(
        "{}/v1/namespaces/{namespace}/applications/{application}/requests/{request_id}/output",
        configuration.base_path,
        namespace = urlencode(namespace),
        application = urlencode(application),
        request_id = urlencode(request_id)
    );
    let req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<V1DownloadFnOutputPayloadSimpleError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
