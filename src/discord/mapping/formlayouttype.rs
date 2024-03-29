use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FormLayoutType {
    NotSet = 0,
    List = 1,
    Gallery = 2,
}
