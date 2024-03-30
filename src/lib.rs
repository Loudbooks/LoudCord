#![allow(dead_code)]

use std::str::FromStr;
use async_trait::async_trait;
use serde_json::Value;

use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::discord::objects::webhook::execute::Execute;
use crate::http::listener::Listener;

pub mod discord;
pub mod http;
pub mod utils;

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::{BasicListener, CatListener, DogListener};
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
        listener_handler.add_listener(
            "doggo".to_string(),
            Box::new(DogListener {})
        );
        listener_handler.add_listener(
            "kitty".to_string(),
            Box::new(CatListener {})
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
                    .build(),
                ApplicationCommandBuilder::new(
                    "doggo", 
                    "Random doggo!", 
                    ApplicationCommandType::ChatInput,                 
                    vec!(
                        ApplicationInteractionContextType::Guild,
                        ApplicationInteractionContextType::BotDM,
                        ApplicationInteractionContextType::PrivateChannel
                    ))
                    .integration_types(
                        vec!(
                            ApplicationIntegrationType::UserInstall
                        )
                    )
                    .build(),
                ApplicationCommandBuilder::new(
                    "kitty",
                    "Random kitty!",
                    ApplicationCommandType::ChatInput,                 
                    vec!(
                        ApplicationInteractionContextType::Guild,
                        ApplicationInteractionContextType::BotDM,
                        ApplicationInteractionContextType::PrivateChannel
                    ))
                    .integration_types(
                        vec!(
                            ApplicationIntegrationType::UserInstall
                        )
                    )
                    .options(
                        vec!(ApplicationCommandOptionBuilder::new(ApplicationCommandOptionType::Integer , "amount", "How many you want?")
                            .required(false)
                            .build()
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
        if incoming_interaction.data.clone().unwrap().name.is_none() ||  incoming_interaction.data.clone().unwrap().name != Some("say".to_string()) {
            return;
        }
        
        let options = incoming_interaction.data.clone().unwrap().options.unwrap();
        let input = options.get(0).unwrap();

        incoming_interaction.reply(input.clone().value.unwrap().as_str(), incoming_interaction).await;
    }
}

pub(crate) struct DogListener {}

#[async_trait]
impl Listener for DogListener {
    async fn on_message(&self, incoming_interaction: &IncomingInteraction) {
        if !utils::is_command_name("doggo", incoming_interaction) {
            return;
        }
        
        let url = "https://dog.ceo/api/breeds/image/random";
        let response = reqwest::get(url).await.unwrap();
        let response = response.json::<Value>().await.unwrap();
        let image = response["message"].as_str().unwrap();

        incoming_interaction.reply(image, incoming_interaction).await;
    }
}


pub(crate) struct CatListener {}

#[async_trait]
impl Listener for CatListener {
    async fn on_message(&self, incoming_interaction: &IncomingInteraction) {
        if !utils::is_command_name("kitty", incoming_interaction) {
            return;
        }
        
        let amount = extract_amount(incoming_interaction);

        incoming_interaction.defer(incoming_interaction).await;
        
        let url = format!("https://api.thecatapi.com/v1/images/search?limit={}", amount);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", dotenv::var("CAT_API_KEY").unwrap().parse().unwrap());
        
        let client = reqwest::Client::new();
        let response = client.get(url).headers(headers).send().await.unwrap();
        
        let response = response.json::<Value>().await.unwrap();
        let array = response.as_array().unwrap();
        
        let images = array.iter().map(|x| x["url"].as_str().unwrap()).collect::<Vec<&str>>();
        let image_str = images.join("\n");

        incoming_interaction.followup(
            Execute::builder()
                .content(image_str.as_str())
                .build(),
            incoming_interaction
        ).await;
    }
}

fn extract_amount(incoming_interaction: &IncomingInteraction) -> i32 {
    let options = incoming_interaction.data.clone().unwrap().options.unwrap();
    let input = options.get(0);
    
    if input.is_none() || input.unwrap().value.is_none() {
        return 1;
    }
    
    let amount = input.clone().unwrap().value.clone().unwrap();
    i32::from_str(amount.as_str()).unwrap()
}