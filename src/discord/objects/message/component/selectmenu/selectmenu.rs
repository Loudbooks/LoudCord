use serde::{Deserialize, Serialize};
use crate::discord::objects::message::component::selectmenu::selectoption::SelectOption;
use crate::discord::mapping::selectmenutype::SelectMenuType;

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectMenu {
    pub r#type: SelectMenuType,
    pub custom_id: String,
    pub options: Vec<SelectOption>,
}

impl SelectMenu {
    pub fn builder(custom_id: &str, select_menu_type: SelectMenuType) -> SelectMenuBuilder {
        SelectMenuBuilder::new(custom_id, select_menu_type)
    }
}

pub struct SelectMenuBuilder {
    r#type: SelectMenuType,
    custom_id: String,
    options: Vec<SelectOption>,
}

impl SelectMenuBuilder {
    pub fn new(custom_id: &str, select_menu_type: SelectMenuType) -> Self {
        SelectMenuBuilder {
            r#type: select_menu_type,
            custom_id: custom_id.to_string(),
            options: Vec::new(),
        }
    }

    pub fn option(mut self, option: SelectOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn build(self) -> SelectMenu {
        SelectMenu {
            r#type: self.r#type,
            custom_id: self.custom_id,
            options: self.options,
        }
    }
}