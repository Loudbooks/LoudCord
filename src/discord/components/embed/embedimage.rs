use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

impl EmbedImage {
    pub fn new(url: &str) -> EmbedImage {
        EmbedImage {
            url: url.to_string(),
            proxy_url: None,
            height: None,
            width: None,
        }
    }
    
    pub fn new_with_dimensions(url: &str, height: u32, width: u32) -> EmbedImage {
        EmbedImage {
            url: url.to_string(),
            proxy_url: None,
            height: Some(height),
            width: Some(width),
        }
    }
}