use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum StickerFormatType {
    Png = 1,
    Apng = 2,
    Lottie = 3,
    Gift = 4,
}