use serde::{Deserialize, Serialize};
use crate::discord::mapping::componenttype::ComponentType;
use crate::discord::mapping::textinputstyle::TextInputStyle;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextInput {
    pub r#type: u8,
    pub custom_id: String,
    pub style: TextInputStyle,
    pub label: String,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub required: Option<bool>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
}

impl TextInput {
    pub fn builder(custom_id: String, style: TextInputStyle, label: String) -> TextInputBuilder {
        TextInputBuilder::new(custom_id, style, label)
    }
}

pub struct TextInputBuilder {
    r#type: u8,
    custom_id: String,
    style: TextInputStyle,
    label: String,
    min_length: Option<u64>,
    max_length: Option<u64>,
    required: Option<bool>,
    value: Option<String>,
    placeholder: Option<String>,
}

impl TextInputBuilder {
    pub fn new(custom_id: String, style: TextInputStyle, label: String) -> Self {
        TextInputBuilder {
            r#type: ComponentType::TextInput as u8,
            custom_id,
            style,
            label,
            min_length: None,
            max_length: None,
            required: None,
            value: None,
            placeholder: None,
        }
    }

    pub fn min_length(mut self, min_length: u64) -> Self {
        self.min_length = Some(min_length);
        self
    }

    pub fn max_length(mut self, max_length: u64) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn build(self) -> TextInput {
        TextInput {
            r#type: self.r#type,
            custom_id: self.custom_id,
            style: self.style,
            label: self.label,
            min_length: self.min_length,
            max_length: self.max_length,
            required: self.required,
            value: self.value,
            placeholder: self.placeholder,
        }
    }
}