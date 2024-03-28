use serde::{Deserialize, Serialize};
use crate::discord::components::attachment::Attachment;
use crate::discord::components::embed::embed::Embed;
use crate::discord::mapping::responseflags::ResponseFlag;

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractionCallback {
    pub tts: Option<bool>,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub allowed_mentions: Option<serde_json::Value>,
    pub flags: Option<i32>,
    pub components: Option<Vec<serde_json::Value>>,
    pub attachments: Option<Vec<Attachment>>,
}


impl InteractionCallback {
    pub fn builder() -> InteractionCallbackBuilder {
        InteractionCallbackBuilder::new()
    }
}

pub struct InteractionCallbackBuilder {
    tts: Option<bool>,
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<serde_json::Value>,
    flags: Option<i32>,
    components: Option<Vec<serde_json::Value>>,
    attachments: Option<Vec<Attachment>>,
}

impl InteractionCallbackBuilder {
    pub fn new() -> Self {
        InteractionCallbackBuilder {
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

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = Some(embeds);
        self
    }

    pub fn allowed_mentions(mut self, allowed_mentions: serde_json::Value) -> Self {
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

    pub fn components(mut self, components: Vec<serde_json::Value>) -> Self {
        self.components = Some(components);
        self
    }

    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn build(self) -> InteractionCallback {
        InteractionCallback {
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