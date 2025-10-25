use bytes::Bytes;
use reqwest::header::HeaderValue;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DownloadOutput {
    pub content: Bytes,
    pub content_type: Option<HeaderValue>,
    pub content_length: Option<HeaderValue>,
}
