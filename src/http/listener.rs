use async_trait::async_trait;
use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::discord::objects::interaction::interactionresponse::InteractionResponse;
use crate::discord::objects::interaction::interactionresponsedata::InteractionResponseData;
use crate::discord::mapping::responsetype::ResponseType;
use crate::http::listenerhandler;

#[async_trait]
pub trait Listener {
    async fn on_message(&self, incoming_interaction: &IncomingInteraction);

    async fn reply(&self, response: &str, incoming_interaction: &IncomingInteraction) {
        let response = InteractionResponse {
            r#type: ResponseType::Message,
            data: Some(InteractionResponseData::builder()
                .content(response)
                .build())
        };

        listenerhandler::ListenerHandler::respond(response, incoming_interaction).await;
    }
    
    async fn defer(&self, incoming_interaction: &IncomingInteraction) {
        let response = InteractionResponse {
            r#type: ResponseType::DeferredMessage,
            data: None
        };
        
        listenerhandler::ListenerHandler::respond(response, incoming_interaction).await;
    }
    
    async fn edit(&self, response: &str, incoming_interaction: &IncomingInteraction) {
        let response = InteractionResponse {
            r#type: ResponseType::DeferredUpdateMessage,
            data: Some(InteractionResponseData::builder()
                .content(response)
                .build())
        };
        
        listenerhandler::ListenerHandler::respond(response, incoming_interaction).await;
    }

    async fn interaction_callback(&self, response: InteractionResponse, incoming_interaction: &IncomingInteraction) {
        listenerhandler::ListenerHandler::respond(response, incoming_interaction).await;
    }
    
    async fn respond(&self, response: InteractionResponse, incoming_interaction: &IncomingInteraction) {
        listenerhandler::ListenerHandler::respond(response, incoming_interaction).await;
    }
}