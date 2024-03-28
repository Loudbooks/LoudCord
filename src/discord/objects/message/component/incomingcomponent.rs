use serde::{Deserialize, Serialize};
use crate::discord::mapping::buttonstyle::ButtonStyle;
use crate::discord::objects::emoji::emoji::Emoji;
use crate::discord::objects::message::component::selectmenu::selectoption::SelectOption;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomingContent {
    pub r#type: u8,
    pub style: ButtonStyle,
    pub label: Option<String>,
    pub emoji: Option<Emoji>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    pub disabled: Option<bool>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub required: Option<bool>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub options: Vec<SelectOption>,
}