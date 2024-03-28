use async_trait::async_trait;
use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::discord::objects::interaction::interactionresponse::InteractionResponse;
use crate::discord::objects::interaction::interactionresponsedata::InteractionResponseData;
use crate::discord::mapping::responsetype::ResponseType;
use crate::http::listenerhandler;

#[async_trait]
pub trait Listener {
    async fn on_message(&self, discord_message: &IncomingInteraction);

    async fn reply(&self, response: &str, discord_message: &IncomingInteraction) {
        let response = InteractionResponse {
            r#type: ResponseType::Message,
            data: InteractionResponseData::builder()
                .content(response)
                .build()
        };

        listenerhandler::ListenerHandler::respond(response, discord_message).await;
    }

    async fn interaction_callback(&self, response: InteractionResponse, discord_message: &IncomingInteraction) {
        println!("{}", serde_json::to_string(&response).unwrap());
        
        listenerhandler::ListenerHandler::respond(response, discord_message).await;
    }
}