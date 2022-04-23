mod incoming;
mod message;
use std::{io::Write, net::TcpStream};

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
    pub fn subscribe(&mut self, channel: &String) {
        let msg = Message::subscribe(channel);
        let msg_str = ron::to_string(&msg).unwrap();
        let _ = self.conn.write_all(msg_str.as_bytes());
        self.conn.flush().unwrap();
    }
    pub fn unsubscribe(&mut self, channel: &String) {
        let msg = Message::unsubscribe(channel);
        let msg_str = ron::to_string(&msg).unwrap();
        let _ = self.conn.write_all(msg_str.as_bytes());
        self.conn.flush().unwrap();
    }

    pub fn publish(&mut self, channel: &String, data: &String) {
        let msg = Message::publish(channel, data);
        let msg_str = ron::to_string(&msg).unwrap();
        let _ = self.conn.write_all(msg_str.as_bytes());
        self.conn.flush().unwrap();
    }
    pub fn recieve_message(&mut self) -> Incoming {
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
