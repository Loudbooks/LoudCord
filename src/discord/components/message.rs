use serde::Deserialize;
use crate::discord::components::attachment::Attachment;
use crate::discord::components::channel::Channel;
use crate::discord::components::channelmention::ChannelMention;
use crate::discord::components::embed::embed::Embed;
use crate::discord::components::messageinteraction::MessageInteractionMetadata;
use crate::discord::components::reaction::reaction::Reaction;
use crate::discord::components::user::User;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: Option<User>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<String>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<String>,
    pub flags: Option<i32>,
    pub interaction_metadata: Option<MessageInteractionMetadata>,
    pub thread: Option<Channel>,
    // pub components: Option<Vec<MessageComponent>>,
    // pub sticker_items: Option<Vec<MessageStickerItem>>,
    // pub stickers: Option<Vec<Sticker>>,
    pub position: Option<i32>,
}




