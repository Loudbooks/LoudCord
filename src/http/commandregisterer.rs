use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap};

use crate::discord::components::command::applicationcommand::ApplicationCommand;

pub async fn register_commands(bot_token: &str, application_id: &str, application_commands: Vec<ApplicationCommand>) {
    let url = format!("applications/{}/commands", application_id);

    discord_request(&url, bot_token, serde_json::to_string(&application_commands).expect("Failed to unwrap application_command")).await;
}

async fn discord_request(endpoint: &str, bot_token: &str, json: String) {
    let url = format!("https://discord.com/api/v10/{}", endpoint);

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bot {}", bot_token).parse().unwrap());
    headers.insert("User-Agent", "DiscordBot (https://github.com/Loudbooks/LoudCord)".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    println!("Request: {}", json);
    
    {
        let this = client
            .put(&url)
            .headers(headers)
            .body(json)
            .send()
            .await;
        match this {
            Ok(_response) =>  {
                println!("Response: {:?}", _response);
            }
            Err(e) => panic!("{:?}", e)
        }
    };
}