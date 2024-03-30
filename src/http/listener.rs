use async_trait::async_trait;
use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;

#[async_trait]
pub trait Listener {
    async fn on_message(&self, incoming_interaction: &IncomingInteraction);
}