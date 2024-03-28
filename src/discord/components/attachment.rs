use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: i64,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: String,
    pub size: i64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub ephemeral: Option<bool>,
    pub duration_seconds: Option<i32>,
    pub waveform: Option<String>,
    pub flags: Option<i32>,
}