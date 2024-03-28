use serde::Deserialize;
use serde_json::Value;
use crate::discord::objects::channel::channel::Channel;
use crate::discord::objects::channel::channelmention::ChannelMention;
use crate::discord::objects::embed::embed::Embed;
use crate::discord::objects::emoji::reaction::Reaction;
use crate::discord::objects::message::attachment::Attachment;
use crate::discord::objects::message::component::button::Button;
use crate::discord::objects::message::messageinteraction::MessageInteractionMetadata;
use crate::discord::objects::message::sticker::Sticker;
use crate::discord::objects::message::stickeritem::StickerItem;
use crate::discord::objects::user::user::User;

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
    pub objects: Option<Vec<Value>>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub stickers: Option<Vec<Sticker>>,
    pub position: Option<i32>,
}




