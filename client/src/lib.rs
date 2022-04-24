mod incoming;
mod message;
use std::{
    io::{BufWriter, Write},
    net::TcpStream,
};

use incoming::Incoming;
use message::Message;

pub struct Client {
    pub conn: TcpStream,
}
impl Client {
    pub fn new(addr: &str) -> Self {
        let conn = TcpStream::connect(addr).unwrap();
        Self { conn }
    }
    pub fn subscribe(&self, channel: &String) {
        let msg = Message::subscribe(channel);
        let msg_str = ron::to_string(&msg).unwrap();
        let mut writer = BufWriter::new(&self.conn);
        let _ = writer.write_all(msg_str.as_bytes());
        writer.flush().unwrap();
    }
    pub fn unsubscribe(&self, channel: &String) {
        let msg = Message::unsubscribe(channel);
        let msg_str = ron::to_string(&msg).unwrap();
        let mut writer = BufWriter::new(&self.conn);
        let _ = writer.write_all(msg_str.as_bytes());
        writer.flush().unwrap();
    }

    pub fn publish(&self, channel: &String, data: &String) {
        let msg = Message::publish(channel, data);
        let msg_str = ron::to_string(&msg).unwrap();
        let mut writer = BufWriter::new(&self.conn);
        let _ = writer.write_all(msg_str.as_bytes());
        writer.flush().unwrap();
    }
    pub fn recieve_message(&self) -> Incoming {
        Incoming { client: self }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
