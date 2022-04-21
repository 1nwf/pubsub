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

impl Message {
    pub fn subscribe(channel: String) -> Message {
        Message {
            kind: MessageType::Subscribe,
            channel,
            data: None,
        }
    }
    pub fn unsubscribe(channel: String) -> Message {
        Message {
            kind: MessageType::Unsubscribe,
            channel,
            data: None,
        }
    }

    pub fn publish(channel: String, data: String) -> Message {
        Message {
            kind: MessageType::Publish,
            channel,
            data: Some(data),
        }
    }
}
