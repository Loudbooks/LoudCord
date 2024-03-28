use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}