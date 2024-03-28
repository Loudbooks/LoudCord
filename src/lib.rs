pub mod discord;
pub mod http;
pub mod listeners;

#[cfg(test)]
mod tests {
    use crate::http::httplistener::HttpListener;
    use crate::http::listenerhandler::ListenerHandler;
    use crate::listeners;

    #[tokio::test]
    async fn main() {
        let mut listener_handler: ListenerHandler = ListenerHandler::new();

        listener_handler.add_listener(
            "test".to_string(),
            Box::new(listeners::basiclistener::BasicListener {})
        );

        let listener: HttpListener = HttpListener { listener_handler };

        /*commandregisterer::register_commands(
            "token",
            "id",
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
        ).await;*/

        listener.start().await.unwrap();
    }
}
