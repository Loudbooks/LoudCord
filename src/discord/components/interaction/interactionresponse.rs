use serde::Serialize;
use crate::discord::components::interaction::interactionresponsedata::InteractionResponseData;
use crate::discord::mapping::responsetype::ResponseType;

#[derive(Serialize)]
pub struct InteractionResponse {
    pub r#type: ResponseType,
    pub data: InteractionResponseData
}