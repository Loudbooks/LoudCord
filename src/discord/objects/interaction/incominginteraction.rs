use serde::Deserialize;
use crate::discord::objects::channel::channel::Channel;
use crate::discord::objects::user::user::User;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct IncomingInteraction {
    pub app_permissions: Option<String>,
    pub application_id: Option<String>,
    pub channel: Option<Channel>,
    pub channel_id: Option<String>,
    pub context: Option<i32>,
    pub data: Option<Data>,
    pub entitlements: Vec<String>,
    pub id: Option<String>,
    pub locale: Option<String>,
    pub token: Option<String>,
    pub r#type: Option<i32>,
    pub user: Option<User>,
    pub version: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Data {
    pub id: Option<String>,
    pub name: Option<String>,
    pub options: Option<Vec<Option<String>>>,
    pub r#type: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Options {
    pub name: Option<String>,
    pub r#type: Option<i32>,
    pub value: Option<String>,
}