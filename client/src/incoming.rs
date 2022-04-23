use std::io::Read;

use crate::Client;

pub struct Incoming<'a> {
    pub client: &'a mut Client,
}

impl<'a> Iterator for Incoming<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 128];
        let n = self.client.conn.read(&mut buf).unwrap();
        let str_data = String::from_utf8(buf[..n].into()).unwrap();
        Some(str_data)
    }
}
