use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use crate::discord::mapping::applicationcommandoptiontype::ApplicationCommandOptionType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    #[serde(deserialize_with = "deserialize_as_string")]
    pub value: Option<String>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub focused: Option<bool>,
    pub r#type: Option<ApplicationCommandOptionType>,
}

fn deserialize_as_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => Some(s),
        Value::Number(num) => Some(num.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    })
}