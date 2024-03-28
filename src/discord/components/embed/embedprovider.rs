use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

impl EmbedProvider {
    pub fn new(name: &str, url: &str) -> EmbedProvider {
        EmbedProvider {
            name: Some(name.to_string()),
            url: Some(url.to_string()),
        }
    }
}