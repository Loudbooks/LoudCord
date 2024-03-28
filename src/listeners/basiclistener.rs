use async_trait::async_trait;
use crate::discord::objects::embed::embed::EmbedBuilder;
use crate::discord::objects::embed::embedfield::EmbedField;
use crate::discord::objects::embed::embedfooter::EmbedFooter;
use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::discord::objects::interaction::interactionresponse::InteractionResponse;
use crate::discord::objects::interaction::interactionresponsedata::InteractionResponseData;
use crate::discord::mapping::responsetype::ResponseType;
use crate::http::listener::Listener;

pub(crate) struct BasicListener {}

#[async_trait]
impl Listener for BasicListener {
    async fn on_message(&self, discord_message: &IncomingInteraction) {
        let response: InteractionResponse = InteractionResponse {
            r#type: ResponseType::Message,
            data: InteractionResponseData::builder()
                .embeds(vec!(
                    EmbedBuilder::new()
                        .title("Title")
                        .description("Description")
                        .color(0x00FF00)
                        .footer(EmbedFooter::new("Footer"))
                        .fields(vec!(
                            EmbedField::new("Field 1", "Value 1", true),
                            EmbedField::new("Field 2", "Value 2", true),
                            EmbedField::new("Field 3", "Value 3", true),
                        ))
                        .build()
                ))
                .build()
        };
        
        self.interaction_callback(response, discord_message).await;
    }
}