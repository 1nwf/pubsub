use std::io::{BufReader, Read};

use crate::Client;

pub struct Incoming<'a> {
    pub client: &'a Client,
}

impl<'a> Iterator for Incoming<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 128];
        let mut reader = BufReader::new(&self.client.conn);
        let n = reader.read(&mut buf).unwrap();
        let str_data = String::from_utf8(buf[..n].into()).unwrap();
        let mut chars = str_data.chars();
        chars.next();
        chars.next_back();
        Some(String::from(chars.as_str()))
    }
}
