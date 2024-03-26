use serde_json::json;
use crate::http::listener::Listener;

pub(crate) struct BasicListener {}

impl Listener for BasicListener {
    fn on_message(&self, _discord_message: &crate::http::discordmessage::DiscordMessage) -> serde_json::Value {
        json!({
            "type": 4,
            "data": {
                "embeds": [
                    {
                        "type": "rich",
                        "title": "MysticExplorer23 Profile (lvl 17)",
                        "color": 9866143,
                        "url": "https://discord.com/developers/docs/intro",
                        "thumbnail": {
                            "url": "https://raw.githubusercontent.com/shaydewael/example-app/main/assets/fake-icon.png"
                        }
                    }
                ]
            }
        })
    }
}