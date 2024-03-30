use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::discord::objects::webhook::execute::Execute;
use crate::http::httphandler::make_request;

pub async fn create_callback(execute: Execute, incoming_interaction: &IncomingInteraction) { 
    let url = format!(
        "https://discord.com/api/v10/webhooks/{}/{}",
        incoming_interaction.application_id.clone().unwrap(),
        incoming_interaction.token.clone().unwrap()
    );
    
    let result = make_request(url, serde_json::to_string(&execute).unwrap()).await;
    
    match result {
        Ok(response) => println!("{}", response),
        Err(e) => println!("{:?}", e),
    }
}