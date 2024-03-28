use serde::{Deserialize, Serialize};
use crate::discord::components::emoji::emoji::Emoji;
use crate::discord::mapping::buttonstyle::ButtonStyle;
use crate::discord::mapping::componenttype::ComponentType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    pub r#type: u8,
    pub style: ButtonStyle,
    pub label: Option<String>,
    pub emoji: Option<Emoji>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    pub disabled: Option<bool>,
}

impl Button {
    pub fn builder(style: ButtonStyle) -> ButtonBuilder {
        ButtonBuilder::new(style)
    }
}

pub struct ButtonBuilder {
    r#type: u8,
    style: ButtonStyle,
    label: Option<String>,
    emoji: Option<Emoji>,
    custom_id: Option<String>,
    url: Option<String>,
    disabled: Option<bool>,
}

impl ButtonBuilder {
    pub fn new(style: ButtonStyle) -> Self {
        ButtonBuilder {
            r#type: ComponentType::Button as u8,
            style,
            label: None,
            emoji: None,
            custom_id: None,
            url: None,
            disabled: None,
        }
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn emoji(mut self, emoji: Emoji) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn custom_id(mut self, custom_id: String) -> Self {
        self.custom_id = Some(custom_id);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn build(self) -> Button {
        Button {
            r#type: self.r#type,
            style: self.style,
            label: self.label,
            emoji: self.emoji,
            custom_id: self.custom_id,
            url: self.url,
            disabled: self.disabled,
        }
    }
}