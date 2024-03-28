use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
    pub text: Option<String>,
}