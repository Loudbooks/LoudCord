use serde::Deserialize;

use crate::discord::components::user::User;

#[derive(Debug, Deserialize)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: Option<String>,
    pub roles: Option<Vec<u64>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}