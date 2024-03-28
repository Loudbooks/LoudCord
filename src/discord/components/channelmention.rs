use serde::Deserialize;
use crate::discord::mapping::channeltype::ChannelType;

#[derive(Debug, Deserialize)]
pub struct ChannelMention {
    pub id: String,
    pub guild_id: String,
    pub r#type: ChannelType,
    pub name: String,
}