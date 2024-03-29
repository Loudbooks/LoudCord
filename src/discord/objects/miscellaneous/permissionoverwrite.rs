use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Overwrite {
    pub id: String,
    pub r#type: String,
    pub allow: String,
    pub deny: String,
}