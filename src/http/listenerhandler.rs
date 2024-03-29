use std::collections::HashMap;
use reqwest::Client;
use reqwest::header::{ CONTENT_TYPE, HeaderMap, HeaderValue };
use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::discord::objects::interaction::interactionresponse::InteractionResponse;
use crate::http::listener::Listener;

pub struct ListenerHandler {
    listeners: HashMap<String, Box<dyn Listener>>,
}

impl ListenerHandler {
    pub fn new() -> ListenerHandler {
        ListenerHandler {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener(&mut self, command: String, listener: Box<dyn Listener>) {
        self.listeners.insert(command, listener);
    }

    pub(crate) async fn handle_message(&self, incoming_interaction: &IncomingInteraction) {
        for listener in self.listeners.iter() {
            listener.1.on_message(incoming_interaction).await;
        }
    }

    pub async fn respond(response: InteractionResponse, incoming_interaction: &IncomingInteraction) {
        let url = format!(
            "https://discord.com/api/v10/interactions/{}/{}/callback",
            incoming_interaction.id.as_ref().unwrap(),
            incoming_interaction.token.as_ref().unwrap()
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
            let this = client.post(&url).headers(headers).body(response_body).send().await;
            match this {
                Ok(_response) => {
                    println!("{}",_response.text().await.unwrap())
                }
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}
