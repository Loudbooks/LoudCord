use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub avatar: Option<String>,
    pub discriminator: Option<String>,
    pub global_name: Option<String>,
    pub id: Option<String>,
    pub public_flags: Option<i32>,
    pub username: Option<String>,
}