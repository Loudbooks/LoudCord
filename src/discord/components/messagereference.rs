use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageReference {
    pub message_id: Option<String>,
    pub channel_id: Option<String>,
    pub guild_id: Option<String>,
    pub fail_if_not_exists: Option<bool>,
}