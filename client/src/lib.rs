mod message;
use std::net::TcpStream;

use message::Message;

pub struct Client {
    conn: TcpStream,
}
impl Client {
    pub fn new(addr: &str) -> Self {
        let conn = TcpStream::connect(addr).unwrap();
        Self { conn }
    }
    pub fn subscribe(channel: String) {}
    pub fn unsubscribe(channel: String) {}

    pub fn publish(channel: String, data: String) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
