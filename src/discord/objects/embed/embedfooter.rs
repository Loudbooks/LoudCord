use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
    pub text: Option<String>,
}

impl EmbedFooter {
    pub fn new(text: &str) -> EmbedFooter {
        EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: Some(text.to_string()),
        }
    }
    
    pub fn new_with_icon(text: &str, icon_url: &str) -> EmbedFooter {
        EmbedFooter {
            icon_url: Some(icon_url.to_string()),
            proxy_icon_url: None,
            text: Some(text.to_string()),
        }
    }
}