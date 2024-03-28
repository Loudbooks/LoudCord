use serde::Deserialize;
use crate::discord::mapping::interactiontype::InteractionType;

#[derive(Debug, Deserialize)]
pub struct MessageInteractionMetadata {
    pub id: String,
    pub r#type: InteractionType,
    pub user_id: String,
    pub original_message_id: Option<String>,
    pub interacted_message_id: Option<String>,
    pub triggered_message_id: Option<String>,
}