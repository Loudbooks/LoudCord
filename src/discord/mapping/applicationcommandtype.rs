use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}
