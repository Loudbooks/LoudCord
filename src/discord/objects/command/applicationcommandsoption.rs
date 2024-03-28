use std::collections::HashMap;
use serde::Serialize;
use crate::discord::objects::command::applicationcommandoptionchoice::ApplicationCommandOptionChoice;
use crate::discord::mapping::applicationcommandoptiontype::ApplicationCommandOptionType;
use crate::discord::mapping::channeltype::ChannelType;

#[derive(Debug, Serialize)]
pub struct ApplicationCommandOption {
    r#type: ApplicationCommandOptionType, 
    name: String, 
    name_localizations: Option<HashMap<String, String>>, 
    description: String, 
    description_localizations: Option<HashMap<String, String>>, 
    required: Option<bool>, 
    choices: Option<Vec<ApplicationCommandOptionChoice>>, 
    options: Option<Vec<ApplicationCommandOption>>, 
    channel_types: Option<Vec<ChannelType>>, 
    min_value: Option<f64>, 
    max_value: Option<f64>, 
    min_length: Option<u64>, 
    max_length: Option<u64>, 
    autocomplete: Option<bool>, 
}

pub struct ApplicationCommandOptionBuilder {
    r#type: ApplicationCommandOptionType, 
    name: String, 
    name_localizations: Option<HashMap<String, String>>, 
    description: String, 
    description_localizations: Option<HashMap<String, String>>, 
    required: Option<bool>, 
    choices: Option<Vec<ApplicationCommandOptionChoice>>, 
    options: Option<Vec<ApplicationCommandOption>>, 
    channel_types: Option<Vec<ChannelType>>, 
    min_value: Option<f64>, 
    max_value: Option<f64>, 
    min_length: Option<u64>, 
    max_length: Option<u64>, 
    autocomplete: Option<bool>, 
}

impl ApplicationCommandOptionBuilder {
    pub fn new(r#type: ApplicationCommandOptionType, name: &str, description: &str) -> ApplicationCommandOptionBuilder {
        ApplicationCommandOptionBuilder {
            r#type,
            name: name.to_string(),
            name_localizations: None,
            description: description.to_string(),
            description_localizations: None,
            required: None,
            choices: None,
            options: None,
            channel_types: None,
            min_value: None,
            max_value: None,
            min_length: None,
            max_length: None,
            autocomplete: None,
        }
    }

    pub fn r#type(mut self, r#type: ApplicationCommandOptionType) -> ApplicationCommandOptionBuilder {
        self.r#type = r#type;
        self
    }

    pub fn name(mut self, name: &str) -> ApplicationCommandOptionBuilder {
        self.name = name.to_string();
        self
    }

    pub fn name_localizations(mut self, name_localizations: HashMap<String, String>) -> ApplicationCommandOptionBuilder {
        self.name_localizations = Some(name_localizations);
        self
    }

    pub fn description(mut self, description: &str) -> ApplicationCommandOptionBuilder {
        self.description = description.to_string();
        self
    }

    pub fn description_localizations(mut self, description_localizations: HashMap<String, String>) -> ApplicationCommandOptionBuilder {
        self.description_localizations = Some(description_localizations);
        self
    }

    pub fn required(mut self, required: bool) -> ApplicationCommandOptionBuilder {
        self.required = Some(required);
        self
    }

    pub fn choices(mut self, choices: Vec<ApplicationCommandOptionChoice>) -> ApplicationCommandOptionBuilder {
        self.choices = Some(choices);
        self
    }

    pub fn options(mut self, options: Vec<ApplicationCommandOption>) -> ApplicationCommandOptionBuilder {
        self.options = Some(options);
        self
    }

    pub fn channel_types(mut self, channel_types: Vec<ChannelType>) -> ApplicationCommandOptionBuilder {
        self.channel_types = Some(channel_types);
        self
    }

    pub fn min_value(mut self, min_value: f64) -> ApplicationCommandOptionBuilder {
        self.min_value = Some(min_value);
        self
    }

    pub fn max_value(mut self, max_value: f64) -> ApplicationCommandOptionBuilder {
        self.max_value = Some(max_value);
        self
    }
    
    pub fn min_length(mut self, min_length: u64) -> ApplicationCommandOptionBuilder {
        self.min_length = Some(min_length);
        self
    }
    
    pub fn max_length(mut self, max_length: u64) -> ApplicationCommandOptionBuilder {
        self.max_length = Some(max_length);
        self
    }
    
    pub fn autocomplete(mut self, autocomplete: bool) -> ApplicationCommandOptionBuilder {
        self.autocomplete = Some(autocomplete);
        self
    }
    
    pub fn build(self) -> ApplicationCommandOption {
        ApplicationCommandOption {
            r#type: self.r#type,
            name: self.name,
            name_localizations: self.name_localizations,
            description: self.description,
            description_localizations: self.description_localizations,
            required: self.required,
            choices: self.choices,
            options: self.options,
            channel_types: self.channel_types,
            min_value: self.min_value,
            max_value: self.max_value,
            min_length: self.min_length,
            max_length: self.max_length,
            autocomplete: self.autocomplete,
        }
    }
}