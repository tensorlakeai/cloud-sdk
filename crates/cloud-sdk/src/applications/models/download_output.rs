use bytes::Bytes;
use reqwest::header::HeaderValue;

pub struct DownloadOutput {
    pub content: Bytes,
    pub content_type: Option<HeaderValue>,
    pub content_length: Option<HeaderValue>,
}
