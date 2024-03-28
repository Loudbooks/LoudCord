use serde::Serialize;
use crate::discord::components::interactioncallback::InteractionCallback;
use crate::discord::mapping::responsetype::ResponseType;

#[derive(Serialize)]
pub struct InteractionResponse {
    pub r#type: ResponseType,
    pub data: InteractionCallback
}