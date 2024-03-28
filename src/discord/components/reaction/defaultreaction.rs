use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DefaultReaction {
    pub emoji_name: String,
    pub emoji_id: String,
}
