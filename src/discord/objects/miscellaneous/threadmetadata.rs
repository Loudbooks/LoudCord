use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: i32,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<String>,
}
