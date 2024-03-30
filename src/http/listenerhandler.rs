use std::collections::HashMap;

use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;
use crate::http::listener::Listener;

pub struct ListenerHandler {
    listeners: HashMap<String, Box<dyn Listener>>,
}

impl ListenerHandler {
    pub fn new() -> ListenerHandler {
        ListenerHandler {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener(&mut self, command: String, listener: Box<dyn Listener>) {
        self.listeners.insert(command, listener);
    }

    pub(crate) async fn handle_message(&self, incoming_interaction: &IncomingInteraction) {
        for listener in self.listeners.iter() {
            listener.1.on_message(incoming_interaction).await;
        }
    }
}
