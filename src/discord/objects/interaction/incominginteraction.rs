use serde::{Deserialize, Serialize};
use crate::discord::mapping::applicationcommandoptiontype::ApplicationCommandOptionType;
use crate::discord::mapping::applicationcommandtype::ApplicationCommandType;
use crate::discord::mapping::applicationinteractioncontexttype::ApplicationInteractionContextType;
use crate::discord::mapping::interactiontype::InteractionType;
use crate::discord::objects::channel::channel::Channel;
use crate::discord::objects::user::user::User;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomingInteraction {
    pub app_permissions: Option<String>,
    pub application_id: Option<String>,
    pub channel: Option<Channel>,
    pub channel_id: Option<String>,
    pub context: Option<ApplicationInteractionContextType>,
    pub data: Option<Data>,
    pub entitlements: Vec<String>,
    pub id: Option<String>,
    pub locale: Option<String>,
    pub token: Option<String>,
    pub r#type: Option<InteractionType>,
    pub user: Option<User>,
    pub version: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub id: Option<String>,
    pub name: Option<String>,
    pub options: Option<Vec<Option<String>>>,
    pub r#type: Option<ApplicationCommandType>,
    
    // Action row only
    pub custom_id: Option<String>,
    pub component_type: Option<i32>,
    pub values: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    pub name: Option<String>,
    pub r#type: Option<ApplicationCommandOptionType>,
    pub value: Option<String>,
}