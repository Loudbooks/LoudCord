use serde::{Deserialize, Serialize};
use crate::discord::components::embed::embedauthor::EmbedAuthor;
use crate::discord::components::embed::embedfield::EmbedField;
use crate::discord::components::embed::embedfooter::EmbedFooter;
use crate::discord::components::embed::embedimage::EmbedImage;
use crate::discord::components::embed::embedprovider::EmbedProvider;
use crate::discord::components::embed::embedthumbnail::EmbedThumbnail;
use crate::discord::components::embed::embedvideo::EmbedVideo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<i32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}