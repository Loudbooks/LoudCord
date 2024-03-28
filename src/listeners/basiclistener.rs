use async_trait::async_trait;
use crate::discord::components::interactioncallback::InteractionCallback;
use crate::discord::interaction::DiscordMessage;
use crate::discord::interactionresponse::InteractionResponse;
use crate::discord::mapping::responsetype::ResponseType;
use crate::http::listener::Listener;

pub(crate) struct BasicListener {}

#[async_trait]
impl Listener for BasicListener {
    async fn on_message(&self, discord_message: &DiscordMessage) {
        println!("aaa");
        
        let response = InteractionResponse {
            r#type: ResponseType::Message,
            data: InteractionCallback::builder()
                .content("Hello, world!".to_string())
                .build()
        };
        
        self.callback(response, discord_message).await;
    }
}