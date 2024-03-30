use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::discord::objects::response::interactionresponse::InteractionResponse;
use crate::http::httphandler::make_request;

pub async fn callback(response: InteractionResponse, incoming_interaction: &IncomingInteraction) {
    let url = format!(
        "https://discord.com/api/v10/interactions/{}/{}/callback",
        incoming_interaction.id.as_ref().unwrap(),
        incoming_interaction.token.as_ref().unwrap()
    );

    let result = make_request(url, serde_json::to_string(&response).unwrap()).await;

    match result {
        Ok(_) => println!("Successfully sent response!"),
        Err(e) => println!("{:?}", e),
    }
}