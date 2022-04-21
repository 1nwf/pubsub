use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
enum MessageType {
    Subscribe,
    Unsubscribe,
    Publish,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    kind: MessageType,
    channel: String,
    data: Option<String>,
}
