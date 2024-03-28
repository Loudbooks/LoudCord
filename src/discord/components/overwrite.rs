use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Overwrite {
    pub id: String,
    pub r#type: String,
    pub allow: String,
    pub deny: String,
}