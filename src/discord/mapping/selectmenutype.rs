use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum SelectMenuType {
    String = 3,
    User = 5,
    Role = 6,
    Mention = 7,
    Channel = 8,
}