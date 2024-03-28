use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

impl EmbedAuthor {
    pub fn new(name: &str) -> EmbedAuthor {
        EmbedAuthor {
            name: Some(name.to_string()),
            url: None,
            icon_url: None,
            proxy_icon_url: None,
        }
    }
    
    pub fn new_with_icon(name: &str, icon_url: &str) -> EmbedAuthor {
        EmbedAuthor {
            name: Some(name.to_string()),
            url: None,
            icon_url: Some(icon_url.to_string()),
            proxy_icon_url: None,
        }
    }
    
    pub fn new_with_url(name: &str, url: &str) -> EmbedAuthor {
        EmbedAuthor {
            name: Some(name.to_string()),
            url: Some(url.to_string()),
            icon_url: None,
            proxy_icon_url: None,
        }
    }
    
    pub fn new_with_icon_and_url(name: &str, icon_url: &str, url: &str) -> EmbedAuthor {
        EmbedAuthor {
            name: Some(name.to_string()),
            url: Some(url.to_string()),
            icon_url: Some(icon_url.to_string()),
            proxy_icon_url: None,
        }
    }
}