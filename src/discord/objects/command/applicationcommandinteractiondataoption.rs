use serde::{Deserialize, Serialize};
use crate::discord::mapping::applicationcommandoptiontype::ApplicationCommandOptionType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    pub value: Option<String>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub focused: Option<bool>,
    pub r#type: Option<ApplicationCommandOptionType>,
}