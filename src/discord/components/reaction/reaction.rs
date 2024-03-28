use serde::Deserialize;

use crate::discord::components::emoji::Emoji;
use crate::discord::components::reaction::reactioncountdetails::ReactionCountDetails;

#[derive(Debug, Deserialize)]
pub struct Reaction {
    pub count: i64,
    pub count_details: ReactionCountDetails,
    pub me: bool,
    pub me_burst: bool,
    pub emoji: Emoji,
    pub burst_colors: Option<Vec<String>>,
}