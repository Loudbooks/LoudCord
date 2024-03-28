use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

impl EmbedField {
    pub fn new(name: &str, value: &str, inline: bool) -> EmbedField {
        EmbedField {
            name: name.to_string(),
            value: value.to_string(),
            inline: Some(inline),
        }
    }
}