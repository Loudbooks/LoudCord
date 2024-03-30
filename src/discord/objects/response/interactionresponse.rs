use serde::Serialize;

use crate::discord::mapping::responsetype::ResponseType;
use crate::discord::objects::response::interactionresponsedata::InteractionResponseData;

#[derive(Serialize)]
pub struct InteractionResponse {
    pub r#type: ResponseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionResponseData>
}