use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AllowedMention {
    pub parse: Vec<String>,
    pub roles: Vec<String>,
    pub users: Vec<String>,
    pub replied_user: bool,
}