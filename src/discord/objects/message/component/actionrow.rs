use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::discord::objects::message::component::button::Button;
use crate::discord::objects::message::component::selectmenu::selectmenu::SelectMenu;
use crate::discord::objects::message::component::textinput::TextInput;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionRow {
    pub r#type: i32,
    pub components: Vec<Value>
}

impl ActionRow {
    pub fn builder() -> ActionRowBuilder {
        ActionRowBuilder::new()
    }
}

pub struct ActionRowBuilder {
    r#type: i32,
    components: Vec<Value>
}

impl ActionRowBuilder {
    pub fn new() -> Self {
        ActionRowBuilder {
            r#type: 1,
            components: Vec::new()
        }
    }

    pub fn r#type(mut self, r#type: i32) -> Self {
        self.r#type = r#type;
        self
    }

    pub fn add_button(mut self, button: Button) -> Self {
        self.components.push(serde_json::to_value(button).unwrap());
        self
    }
    
    pub fn add_select_menu(mut self, select_menu: SelectMenu) -> Self {
        self.components.push(serde_json::to_value(select_menu).unwrap());
        self
    }
    
    pub fn add_text_input(mut self, text_input: TextInput) -> Self {
        self.components.push(serde_json::to_value(text_input).unwrap());
        self
    }
    
    pub fn build(self) -> ActionRow {
        ActionRow {
            r#type: self.r#type,
            components: self.components
        }
    }
}