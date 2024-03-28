use serde::Deserialize;

use crate::discord::objects::emoji::reactioncountdetails::ReactionCountDetails;

use super::emoji::Emoji;

#[derive(Debug, Deserialize)]
pub struct Reaction {
    pub count: i64,
    pub count_details: ReactionCountDetails,
    pub me: bool,
    pub me_burst: bool,
    pub emoji: Emoji,
    pub burst_colors: Option<Vec<String>>,
}