use serde::Deserialize;

use crate::discord::components::emoji::defaultreaction::DefaultReaction;
use crate::discord::components::miscellaneous::permissionoverwrite::Overwrite;
use crate::discord::components::miscellaneous::threadmetadata::ThreadMetadata;
use crate::discord::components::user::guildmember::GuildMember;
use crate::discord::components::user::user::User;
use crate::discord::mapping::{
    channeltype::ChannelType, videoqualitymode::VideoQualityMode,
};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Channel {
    pub id: String,
    pub r#type: ChannelType,
    pub guild_id: Option<String>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityMode>,
    pub message_count: Option<i32>,
    pub member_count: Option<i32>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<GuildMember>,
    pub default_auto_archive_duration: Option<i32>,
    pub permissions: Option<String>,
    pub flags: Option<i32>,
    pub total_message_sent: Option<i32>,
    pub available_tags: Option<String>,
    pub default_reaction_emoji: Option<DefaultReaction>,
}
