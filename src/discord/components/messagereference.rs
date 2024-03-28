use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageReference {
    pub message_id: Option<u64>,
    pub channel_id: Option<u64>,
    pub guild_id: Option<u64>,
    pub fail_if_not_exists: Option<bool>,
}