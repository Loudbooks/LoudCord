use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::discord::mapping::responseflags::ResponseFlag;
use crate::discord::objects::embed::embed::Embed;
use crate::discord::objects::message::attachment::Attachment;
use crate::discord::objects::message::component::actionrow::ActionRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractionResponseData {
    pub tts: Option<bool>,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub allowed_mentions: Option<Value>,
    pub flags: Option<i32>,
    pub components: Option<Vec<ActionRow>>,
    pub attachments: Option<Vec<Attachment>>,
}

impl InteractionResponseData {
    pub fn builder() -> InteractionResponseDataBuilder {
        InteractionResponseDataBuilder::new()
    }
}

pub struct InteractionResponseDataBuilder {
    tts: Option<bool>,
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<Value>,
    flags: Option<i32>,
    components: Option<Vec<ActionRow>>,
    attachments: Option<Vec<Attachment>>,
}

impl InteractionResponseDataBuilder {
    pub fn new() -> Self {
        InteractionResponseDataBuilder {
            tts: None,
            content: None,
            embeds: None,
            allowed_mentions: None,
            flags: Some(0),
            components: None,
            attachments: None,
        }
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = Some(tts);
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = Some(embeds);
        self
    }

    pub fn allowed_mentions(mut self, allowed_mentions: Value) -> Self {
        self.allowed_mentions = Some(allowed_mentions);
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
    
    pub fn add_action_row(mut self, action_row: ActionRow) -> Self {
        let mut current_components = self.components.unwrap_or_default();
        
        current_components.push(action_row);
        self.components = Some(current_components);
        self
    }

    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn build(self) -> InteractionResponseData {
        InteractionResponseData {
            tts: self.tts,
            content: self.content,
            embeds: self.embeds,
            allowed_mentions: self.allowed_mentions,
            flags: self.flags,
            components: self.components,
            attachments: self.attachments,
        }
    }
}