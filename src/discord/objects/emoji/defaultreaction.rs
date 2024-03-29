use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultReaction {
    pub emoji_name: String,
    pub emoji_id: String,
}
