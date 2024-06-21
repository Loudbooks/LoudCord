use reqwest::Method;
use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::discord::objects::webhook::execute::Execute;
use crate::http::httphandler::make_request;

pub async fn create_post_callback(execute: Execute, incoming_interaction: &IncomingInteraction) {
    create_callback(execute, incoming_interaction, "", Method::POST).await;
}

pub async fn create_callback(execute: Execute, incoming_interaction: &IncomingInteraction, endpoint: &str, method: Method) {
    let url = format!(
        "https://discord.com/api/v10/webhooks/{}/{}{}",
        incoming_interaction.application_id.clone().unwrap(),
        incoming_interaction.token.clone().unwrap(),
        endpoint
    );
    println!("{}", url);

    let result = make_request(url, serde_json::to_string(&execute).unwrap(), method).await;
    
    match result {
        Ok(response) => println!("{}", response),
        Err(e) => println!("{:?}", e),
    }
}