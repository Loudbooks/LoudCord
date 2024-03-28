use serde::{Deserialize, Serialize};
use crate::discord::mapping::stickerformattype::StickerFormatType;
use crate::discord::mapping::stickertype::StickerType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sticker {
    pub id: Option<String>,
    pub pack_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub asset: Option<String>,
    pub r#type: Option<StickerType>,
    pub format_type: Option<StickerFormatType>,
    pub available: Option<bool>,
    pub guild_id: Option<String>,
    pub user: Option<String>,
    pub sort_value: Option<i32>,
}