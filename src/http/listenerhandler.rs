use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use crate::http::discordmessage::DiscordMessage;
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
            let response = listener.on_message(discord_message);
            self.respond(&response, discord_message).await;
        }
    }

    async fn respond(&self, response: &serde_json::Value, discord_message: &DiscordMessage) {
        let url = format!(
            "https://discord.com/api/v10/interactions/{}/{}/callback",
            discord_message.id.as_ref().unwrap(),
            discord_message.token.as_ref().unwrap() 
        );

        let response_body = serde_json::to_string(&response).unwrap(); 

        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json")); 

        client
            .post(&url)
            .headers(headers)
            .body(response_body)
            .send().await.unwrap();
    }
}
