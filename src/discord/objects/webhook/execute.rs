use serde::{Deserialize, Serialize};
use crate::discord::mapping::responseflags::ResponseFlag;
use crate::discord::objects::embed::embed::Embed;
use crate::discord::objects::message::component::actionrow::ActionRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Execute {
    pub content: String,
    pub tts: bool,
    pub embeds: Vec<Embed>,
    pub components: Vec<ActionRow>,
    pub flags: Option<i32>,
}

impl Execute {
    pub fn builder() -> ExecuteBuilder {
        ExecuteBuilder::new()
    }
}

pub struct ExecuteBuilder {
    content: String,
    tts: bool,
    embeds: Vec<Embed>,
    components: Vec<ActionRow>,
    flags: Option<i32>,
}

impl ExecuteBuilder {
    pub fn new() -> Self {
        ExecuteBuilder {
            content: String::new(),
            tts: false,
            embeds: Vec::new(),
            components: Vec::new(),
            flags: Some(0),
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = tts;
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = embeds;
        self
    }
    
    pub fn components(mut self, components: Vec<ActionRow>) -> Self {
        self.components = components;
        self
    }

    pub fn add_flag(mut self, flag: ResponseFlag) -> Self {
        let current_value = if self.flags.is_some() {
            self.flags.unwrap()
        } else {
            0
        };

        self.flags = Some(current_value + (flag as i32));
        self
    }

    pub fn build(self) -> Execute {
        Execute {
            content: self.content,
            tts: self.tts,
            embeds: self.embeds,
            components: self.components,
            flags: self.flags,
        }
    }
}