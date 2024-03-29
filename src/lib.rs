#![allow(dead_code)]

use async_trait::async_trait;

use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::http::listener::Listener;

pub mod discord;
pub mod http;

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::BasicListener;
    use crate::discord::mapping::applicationcommandoptiontype::ApplicationCommandOptionType;
    use crate::discord::mapping::applicationcommandtype::ApplicationCommandType;
    use crate::discord::mapping::applicationintegrationtype::ApplicationIntegrationType;
    use crate::discord::mapping::applicationinteractioncontexttype::ApplicationInteractionContextType;
    use crate::discord::objects::command::applicationcommand::ApplicationCommandBuilder;
    use crate::discord::objects::command::applicationcommandsoption::ApplicationCommandOptionBuilder;
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
                    "say",
                    "Says what you want!",
                    ApplicationCommandType::ChatInput,
                    vec!(
                        ApplicationInteractionContextType::Guild,
                        ApplicationInteractionContextType::BotDM,
                        ApplicationInteractionContextType::PrivateChannel
                    )
                )
                    .options(
                        vec!(ApplicationCommandOptionBuilder::new(ApplicationCommandOptionType::String, "string", "What to say")
                            .required(true)
                            .build())
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
    async fn on_message(&self, incoming_interaction: &IncomingInteraction) {
        let options = incoming_interaction.data.clone().unwrap().options.unwrap();
        let input = options.get(0).unwrap();

        self.reply(input.clone().value.unwrap().as_str(), incoming_interaction).await;
    }
}