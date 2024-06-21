use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::discord::mapping::applicationcommandoptiontype::ApplicationCommandOptionType;
use crate::discord::mapping::applicationcommandtype::ApplicationCommandType;
use crate::discord::mapping::applicationinteractioncontexttype::ApplicationInteractionContextType;
use crate::discord::mapping::interactiontype::InteractionType;
use crate::discord::mapping::responsetype::ResponseType;
use crate::discord::objects::channel::channel::Channel;
use crate::discord::objects::command::applicationcommandinteractiondataoption::ApplicationCommandInteractionDataOption;
use crate::discord::objects::response::interactionresponse::InteractionResponse;
use crate::discord::objects::response::interactionresponsedata::InteractionResponseData;
use crate::discord::objects::user::user::User;
use crate::discord::objects::webhook::execute::Execute;
use crate::http::{callbackhandler, webhookhandler};

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
    pub options: Option<Vec<Options>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub id: Option<String>,
    pub name: Option<String>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
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

impl IncomingInteraction {
    pub async fn reply(&self, response: &str, incoming_interaction: &IncomingInteraction) {
        let response = InteractionResponse {
            r#type: ResponseType::Message,
            data: Some(InteractionResponseData::builder()
                .content(response)
                .build())
        };

        self.interaction_callback(response, incoming_interaction).await;
    }

    pub async fn defer(&self, incoming_interaction: &IncomingInteraction) {
        let response = InteractionResponse {
            r#type: ResponseType::DeferredMessage,
            data: None
        };

        self.interaction_callback(response, incoming_interaction).await;
    }

    pub async fn edit(&self, response: &str, incoming_interaction: &IncomingInteraction) {
        let response = InteractionResponse {
            r#type: ResponseType::DeferredUpdateMessage,
            data: Some(InteractionResponseData::builder()
                .content(response)
                .build())
        };

        self.interaction_callback(response, incoming_interaction).await;
    }

    pub async fn post_followup(&self, response: Execute, incoming_interaction: &IncomingInteraction) {
        webhookhandler::create_post_callback(response, incoming_interaction).await;
    }

    pub async fn edit_followup(&self, response: Execute, incoming_interaction: &IncomingInteraction) {
        webhookhandler::create_callback(response, incoming_interaction, "/messages/@original", Method::PATCH).await;
    }

    pub async fn interaction_callback(&self, response: InteractionResponse, incoming_interaction: &IncomingInteraction) {
        callbackhandler::callback(response, incoming_interaction).await;
    }
}