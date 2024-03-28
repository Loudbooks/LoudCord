use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use crate::discord::interaction::DiscordMessage;
use crate::discord::interactionresponse::InteractionResponse;
use crate::http::listener::Listener;


pub struct ListenerHandler {
    listeners: Vec<Box<dyn Listener>>,
}

impl ListenerHandler {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn Listener>) {
        self.listeners.push(listener);
    }

    pub(crate) async fn handle_message(&self, discord_message: &DiscordMessage) {
        for listener in self.listeners.iter() {
            listener.on_message(discord_message).await;
        }
    }

    pub async fn respond(response: InteractionResponse, discord_message: &DiscordMessage) {
        let url = format!(
            "https://discord.com/api/v10/interactions/{}/{}/callback",
            discord_message.id.as_ref().unwrap(),
            discord_message.token.as_ref().unwrap()
        );

        let response_body = {
            let this = serde_json::to_string(&response);
            
            match this {
                Ok(t) => t,
                Err(e) => panic!("{:?}", e),
            }
        };

        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        {
            let this = client
                .post(&url)
                .headers(headers)
                .body(response_body)
                .send()
                .await;
            match this {
                Ok(response) =>  {
                    let response_text = response.text().await.unwrap();
                    println!("Response: {:?}", response_text);
                }
                Err(e) => panic!("{:?}", e)
            }
        };
    }
}
