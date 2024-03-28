use async_trait::async_trait;
use crate::discord::components::interactioncallback::InteractionCallback;
use crate::discord::interaction::DiscordMessage;
use crate::discord::interactionresponse::InteractionResponse;
use crate::discord::mapping::responsetype::ResponseType;
use crate::http::listenerhandler;

#[async_trait]
pub trait Listener {
    async fn on_message(&self, discord_message: &DiscordMessage);

    async fn reply(&self, response: String, discord_message: &DiscordMessage) {
        let response = InteractionResponse {
            r#type: ResponseType::Message,
            data: InteractionCallback::builder()
                .content(response)
                .build()
        };

        listenerhandler::ListenerHandler::respond(response, discord_message).await;
    }

    async fn callback(&self, response: InteractionResponse, discord_message: &DiscordMessage) {
        listenerhandler::ListenerHandler::respond(response, discord_message).await;
    }
}