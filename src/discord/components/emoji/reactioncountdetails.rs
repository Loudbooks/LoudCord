use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionCountDetails {
    pub burst: u32,
    pub normal: u32,
}