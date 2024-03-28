use serde::{Deserialize, Serialize};
use crate::discord::objects::emoji::emoji::Emoji;

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}

impl SelectOption {
    pub fn builder(label: String, value: String) -> SelectOptionBuilder {
        SelectOptionBuilder::new(label, value)
    }
}

pub struct SelectOptionBuilder {
    label: String,
    value: String,
    description: Option<String>,
    emoji: Option<Emoji>,
    default: Option<bool>,
}

impl SelectOptionBuilder {
    pub fn new(label: String, value: String) -> Self {
        SelectOptionBuilder {
            label,
            value,
            description: None,
            emoji: None,
            default: None,
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn emoji(mut self, emoji: Emoji) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }

    pub fn build(self) -> SelectOption {
        SelectOption {
            label: self.label,
            value: self.value,
            description: self.description,
            emoji: self.emoji,
            default: self.default,
        }
    }
}