use crate::http::discordmessage::DiscordMessage;

pub trait Listener {
    fn on_message(&self, discord_message: &DiscordMessage) -> serde_json::Value;
}