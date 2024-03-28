use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmbedType {
    Rich, 
    Image,
    Video,
    Gifv,
    Article,
    Link,
}