use serde::Deserialize;
use crate::discord::mapping::interactiontype::InteractionType;

#[derive(Debug, Deserialize)]
pub struct MessageInteractionMetadata {
    pub id: u64,
    pub r#type: InteractionType,
    pub user_id: u64,
    pub original_message_id: Option<u64>,
    pub interacted_message_id: Option<u64>,
    pub triggered_message_id: Option<u64>,
}