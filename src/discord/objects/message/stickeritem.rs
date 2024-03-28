use serde::{Deserialize, Serialize};
use crate::discord::mapping::stickerformattype::StickerFormatType;

#[derive(Debug, Serialize, Deserialize)]

pub struct StickerItem {
    pub id: Option<String>,
    pub name: Option<String>,
    pub format_type: Option<StickerFormatType>,
}