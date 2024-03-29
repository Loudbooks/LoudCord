#![allow(dead_code)]

use async_trait::async_trait;

use crate::discord::mapping::interactiontype::InteractionType;
use crate::discord::mapping::responsetype::ResponseType;
use crate::discord::mapping::selectmenutype::SelectMenuType;
use crate::discord::objects::embed::embed::EmbedBuilder;
use crate::discord::objects::embed::embedfield::EmbedField;
use crate::discord::objects::embed::embedfooter::EmbedFooter;
use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::discord::objects::interaction::interactionresponse::InteractionResponse;
use crate::discord::objects::interaction::interactionresponsedata::InteractionResponseData;
use crate::discord::objects::message::component::actionrow::ActionRowBuilder;
use crate::discord::objects::message::component::selectmenu::selectmenu::SelectMenuBuilder;
use crate::discord::objects::message::component::selectmenu::selectoption::SelectOption;
use crate::http::listener::Listener;

pub mod discord;
pub mod http;

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use crate::BasicListener;
    use crate::discord::mapping::applicationcommandtype::ApplicationCommandType;
    use crate::discord::mapping::applicationintegrationtype::ApplicationIntegrationType;
    use crate::discord::mapping::applicationinteractioncontexttype::ApplicationInteractionContextType;
    use crate::discord::objects::command::applicationcommand::ApplicationCommandBuilder;
    use crate::http::commandregisterer;
    use crate::http::httplistener::HttpListener;
    use crate::http::listenerhandler::ListenerHandler;

    #[tokio::test]
    async fn main() {
        dotenv().ok();
        let mut listener_handler: ListenerHandler = ListenerHandler::new();

        listener_handler.add_listener(
            "test".to_string(),
            Box::new(BasicListener {})
        );

        let listener: HttpListener = HttpListener { 
            listener_handler,
            public_key: std::env::var("PUBLIC_KEY").expect("PUBLIC_KEY must be set.")
        };

        commandregisterer::register_commands(
            &std::env::var("BOT_TOKEN").expect("BOT_TOKEN must be set."),
            &std::env::var("APPLICATION_ID").expect("APPLICATION_ID must be set."),
            vec!(
                ApplicationCommandBuilder::new(
                    "test",
                    "Its a test!",
                    ApplicationCommandType::ChatInput,
                    vec!(
                        ApplicationInteractionContextType::Guild,
                        ApplicationInteractionContextType::BotDM,
                        ApplicationInteractionContextType::PrivateChannel
                    )
                )
                    .integration_types(
                        vec!(
                            ApplicationIntegrationType::UserInstall
                        )
                    )
                    .build()
            )
        ).await;

        listener.start().await.unwrap();
    }
}

pub(crate) struct BasicListener {}

#[async_trait]
impl Listener for BasicListener {
    async fn on_message(&self, discord_message: &IncomingInteraction) {
        if discord_message.r#type.clone().unwrap() == InteractionType::MessageComponent {
            let selected_option = discord_message.data.clone().unwrap().values.unwrap().get(0).unwrap().clone();
            
            let response: InteractionResponse = InteractionResponse {
                r#type: ResponseType::Message,
                data: Some(InteractionResponseData::builder()
                    .content(
                        format!("You selected: {}", selected_option).as_str()
                    )
                    .build())
            };

            self.interaction_callback(response, discord_message).await;
        } else {
            let response: InteractionResponse = InteractionResponse {
                r#type: ResponseType::Message,
                data: Some(InteractionResponseData::builder()
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
                    .add_action_row(
                        ActionRowBuilder::new().add_select_menu(
                            SelectMenuBuilder::new("test", SelectMenuType::String)
                                .option(
                                    SelectOption::builder("Option 1".to_string(), "option1".to_string())
                                        .description("Description".to_string())
                                        .build()
                                )
                                .option(
                                    SelectOption::builder("Option 2".to_string(), "option2".to_string())
                                        .description("Description".to_string())
                                        .build()
                                )
                                .build()
                        ).build()
                    ).build())
            };

            self.interaction_callback(response, discord_message).await;
        }
    }
}