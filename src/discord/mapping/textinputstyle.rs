use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum TextInputStyle {
    Short = 1,
    Pharaoh = 2,
}