use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub enum MessageType {
    Subscribe,
    Unsubscribe,
    Publish,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub kind: MessageType,
    pub channel: String,
    pub data: Option<String>,
}
