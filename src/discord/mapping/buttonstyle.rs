use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}