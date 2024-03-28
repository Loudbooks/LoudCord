use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedVideo {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

impl EmbedVideo {
    pub fn new(url: &str) -> EmbedVideo {
        EmbedVideo {
            url: url.to_string(),
            proxy_url: None,
            height: None,
            width: None,
        }
    }
    
    pub fn new_with_dimensions(url: &str, height: u32, width: u32) -> EmbedVideo {
        EmbedVideo {
            url: url.to_string(),
            proxy_url: None,
            height: Some(height),
            width: Some(width),
        }
    }
}