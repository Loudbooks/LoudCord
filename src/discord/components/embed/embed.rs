use serde::{Deserialize, Serialize};
use crate::discord::components::embed::embedauthor::EmbedAuthor;
use crate::discord::components::embed::embedfield::EmbedField;
use crate::discord::components::embed::embedfooter::EmbedFooter;
use crate::discord::components::embed::embedimage::EmbedImage;
use crate::discord::components::embed::embedprovider::EmbedProvider;
use crate::discord::components::embed::embedthumbnail::EmbedThumbnail;
use crate::discord::components::embed::embedvideo::EmbedVideo;
use crate::discord::mapping::embedtype::EmbedType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub title: Option<String>,
    pub r#type: Option<EmbedType>,
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

pub struct EmbedBuilder {
    title: Option<String>,
    r#type: Option<EmbedType>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<i32>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>,
}

impl EmbedBuilder {
    pub fn new() -> EmbedBuilder {
        EmbedBuilder {
            title: None,
            r#type: None,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }
    
    pub fn title(mut self, title: &str) -> EmbedBuilder {
        self.title = Some(title.to_string());
        self
    }
    
    pub fn r#type(mut self, r#type: EmbedType) -> EmbedBuilder {
        self.r#type = Some(r#type);
        self
    }
    
    pub fn description(mut self, description: &str) -> EmbedBuilder {
        self.description = Some(description.to_string());
        self
    }
    
    pub fn url(mut self, url: &str) -> EmbedBuilder {
        self.url = Some(url.to_string());
        self
    }
    
    pub fn timestamp(mut self, timestamp: &str) -> EmbedBuilder {
        self.timestamp = Some(timestamp.to_string());
        self
    }
    
    pub fn color(mut self, color: i32) -> EmbedBuilder {
        self.color = Some(color);
        self
    }
    
    pub fn footer(mut self, footer: EmbedFooter) -> EmbedBuilder {
        self.footer = Some(footer);
        self
    }
    
    pub fn image(mut self, image: EmbedImage) -> EmbedBuilder {
        self.image = Some(image);
        self
    }
    
    pub fn thumbnail(mut self, thumbnail: EmbedThumbnail) -> EmbedBuilder {
        self.thumbnail = Some(thumbnail);
        self
    }
    
    pub fn video(mut self, video: EmbedVideo) -> EmbedBuilder {
        self.video = Some(video);
        self
    }
    
    pub fn provider(mut self, provider: EmbedProvider) -> EmbedBuilder {
        self.provider = Some(provider);
        self
    }
    
    pub fn author(mut self, author: EmbedAuthor) -> EmbedBuilder {
        self.author = Some(author);
        self
    }
    
    pub fn fields(mut self, fields: Vec<EmbedField>) -> EmbedBuilder {
        self.fields = Some(fields);
        self
    }
    
    pub fn build(self) -> Embed {
        Embed {
            title: self.title,
            r#type: self.r#type,
            description: self.description,
            url: self.url,
            timestamp: self.timestamp,
            color: self.color,
            footer: self.footer,
            image: self.image,
            thumbnail: self.thumbnail,
            video: self.video,
            provider: self.provider,
            author: self.author,
            fields: self.fields,
        }
    }
}