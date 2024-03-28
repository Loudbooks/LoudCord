use serde::{Deserialize, Serialize};
use crate::discord::objects::message::component::selectmenu::selectdefaultvaluetype::SelectDefaultValueType;

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectDefaultValue {
    pub id: String,
    pub r#type: SelectDefaultValueType
}