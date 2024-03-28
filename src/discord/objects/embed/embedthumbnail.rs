use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

impl EmbedThumbnail {
    pub fn new(url: &str) -> EmbedThumbnail {
        EmbedThumbnail {
            url: url.to_string(),
            proxy_url: None,
            height: None,
            width: None,
        }
    }
    
    pub fn new_with_dimensions(url: &str, height: u32, width: u32) -> EmbedThumbnail {
        EmbedThumbnail {
            url: url.to_string(),
            proxy_url: None,
            height: Some(height),
            width: Some(width),
        }
    }
}