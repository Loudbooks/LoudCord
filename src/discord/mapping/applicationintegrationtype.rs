use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ApplicationIntegrationType {
    GuildInstall = 0,
    UserInstall = 1,
}