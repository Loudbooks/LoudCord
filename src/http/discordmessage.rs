use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DiscordMessage {
    pub(crate) app_permissions: Option<String>,
    pub(crate) application_id: Option<String>,
    pub(crate) channel: Option<Channel>,
    pub(crate) channel_id: Option<String>,
    pub(crate) context: Option<i32>,
    pub(crate) data: Option<Data>,
    pub(crate) entitlements: Option<Vec<String>>,
    pub(crate) id: Option<String>,
    pub(crate) locale: Option<String>,
    pub(crate) token: Option<String>,
    pub(crate) r#type: Option<i32>,
    pub(crate) user: Option<User>,
    pub(crate) version: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Channel {
    pub(crate) flags: Option<i32>,
    pub(crate) icon: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) last_message_id: Option<String>,
    pub(crate) last_pin_timestamp: Option<String>, // Note: Change to appropriate type if needed
    pub(crate) name: Option<String>,
    pub(crate) owner_id: Option<String>,
    pub(crate) r#type: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Data {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) options: Option<Vec<Option<String>>>,
    pub(crate) r#type: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Options {
    pub(crate) name: Option<String>,
    pub(crate) r#type: Option<i32>,
    pub(crate) value: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct User {
    pub(crate) avatar: Option<String>,
    pub(crate) discriminator: Option<String>,
    pub(crate) global_name: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) public_flags: Option<i32>,
    pub(crate) username: Option<String>,
}
