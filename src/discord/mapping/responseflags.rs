use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ResponseFlag {
    EPHEMERAL = 1 << 6,
}
