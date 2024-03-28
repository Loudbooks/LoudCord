use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}