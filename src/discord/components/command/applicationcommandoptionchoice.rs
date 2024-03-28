use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: String,
}

pub struct ApplicationCommandOptionChoiceBuilder {
    name: String,
    name_localizations: Option<HashMap<String, String>>,
    value: String,
}

impl ApplicationCommandOptionChoiceBuilder {
    pub fn new(name: String, value: String) -> ApplicationCommandOptionChoiceBuilder {
        ApplicationCommandOptionChoiceBuilder {
            name,
            name_localizations: None,
            value,
        }
    }

    pub fn name(mut self, name: String) -> ApplicationCommandOptionChoiceBuilder {
        self.name = name;
        self
    }

    pub fn name_localizations(mut self, name_localizations: HashMap<String, String>) -> ApplicationCommandOptionChoiceBuilder {
        self.name_localizations = Some(name_localizations);
        self
    }

    pub fn value(mut self, value: String) -> ApplicationCommandOptionChoiceBuilder {
        self.value = value;
        self
    }

    pub fn build(self) -> ApplicationCommandOptionChoice {
        ApplicationCommandOptionChoice {
            name: self.name,
            name_localizations: self.name_localizations,
            value: self.value,
        }
    }
}