use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum ChannelFlag {
    Pinned = 1 << 1,
    RequireTag = 1 << 4,
    HideMediaDownloadOptions = 1 << 15,
}
