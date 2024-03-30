use crate::discord::objects::interaction::incominginteraction::IncomingInteraction;

pub fn is_command_name(name: &str, incoming_interaction: &IncomingInteraction) -> bool {
    let data = incoming_interaction.data.clone();
    if data.is_none() {
        return false;
    }
    
    let data = data.unwrap();
    if data.name.is_none() {
        return false;
    }
    
    data.name.unwrap() == name
}

pub fn is_custom_id(custom_id: &str, incoming_interaction: &IncomingInteraction) -> bool {
    let data = incoming_interaction.data.clone();
    if data.is_none() {
        return false;
    }
    
    let data = data.unwrap();
    if data.custom_id.is_none() {
        return false;
    }
    
    data.custom_id.unwrap() == custom_id
}