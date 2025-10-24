#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client,
}

impl Configuration {
    pub fn new(base_path: &str, client: reqwest::Client) -> Configuration {
        Configuration {
            base_path: base_path.to_string(),
            client,
        }
    }
}
