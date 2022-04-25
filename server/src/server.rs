pub struct Server {}

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    thread,
};

use ron::from_str;

use crate::message::{Message, MessageType};

use super::SUBSCRIBERS;
impl Server {
    pub fn new() -> Self {
        Self {}
    }
    pub fn start(self) {
        let lis = TcpListener::bind("localhost:6379").unwrap();
        for stream in lis.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        Server::handle_conn(stream);
                    });
                }
                Err(_) => println!("failed to connect"),
            }
        }
    }
    fn subscribe(msg: Message, conn: &TcpStream) {
        let ptr = (conn as *const TcpStream) as usize;
        let mut subscribers = SUBSCRIBERS.lock().unwrap();
        if let std::collections::hash_map::Entry::Vacant(e) = subscribers.entry(msg.channel.clone())
        {
            e.insert(Vec::from([ptr]));
        } else {
            subscribers.get_mut(&msg.channel).unwrap().push(ptr);
        }
    }
    fn unsubscribe(msg: Message, conn: &TcpStream) {
        let ptr = (conn as *const TcpStream) as usize;
        let mut subscribers = SUBSCRIBERS.lock().unwrap();
        if subscribers.contains_key(&msg.channel) {
            let channel_subs = subscribers.get_mut(&msg.channel).unwrap();
            let idx = channel_subs.iter().position(|x| *x == ptr);
            channel_subs.remove(idx.unwrap());
        }
    }

    fn publish(msg: Message) {
        let subs = SUBSCRIBERS.lock().unwrap();
        let subscribed_connections = subs.get(&msg.channel);
        if let Some(connections) = subscribed_connections {
            for addr in connections {
                let conn_ptr = *addr as *mut TcpStream;
                let mut conn = unsafe { &*conn_ptr };
                let str_data = ron::ser::to_string(&msg.data.as_ref().unwrap()).unwrap();
                let str_bytes = str_data.as_bytes();
                conn.write_all(str_bytes).expect("unable to write to buf");
                conn.flush().expect("could not flush client output stream");
            }
        }
    }
    pub fn handle_conn(mut conn: TcpStream) {
        loop {
            let mut buf = [0; 128];
            let n = conn.read(&mut buf).unwrap_or(0);
            let str_data = from_utf8(&buf[..n]).unwrap();
            if n != 0 {
                let msg: Message = from_str(str_data).unwrap();
                println!("recieved {:?}", msg);
                match msg.kind {
                    MessageType::Publish => Server::publish(msg),
                    MessageType::Subscribe => Server::subscribe(msg, &conn),
                    MessageType::Unsubscribe => Server::unsubscribe(msg, &conn),
                }
            }
        }
    }
}
