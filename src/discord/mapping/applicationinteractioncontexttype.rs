use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ApplicationInteractionContextType {
    Guild = 0,
    BotDM = 1,
    PrivateChannel = 2,
}