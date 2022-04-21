mod message;
use message::{Message, MessageType};
use std::{
    io::{BufWriter, Write},
    sync::Mutex,
};

use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    io::Read,
    net::{TcpListener, TcpStream},
};
lazy_static! {
    static ref CHANNELS: HashSet<String> = HashSet::new();
    static ref SUBSCRIBERS: Mutex<HashMap<String, Vec<TcpStream>>> = Mutex::new(HashMap::new());
}
fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:6379";
    let lis = TcpListener::bind(addr)?;
    for stream in lis.incoming() {
        handle_conn(stream?)
    }
    Ok(())
}

fn handle_conn(mut conn: TcpStream) {
    let mut buf = [0; 128];
    let n = conn.read(&mut buf).unwrap();
    handle_buf(&buf[..n], conn);
}

fn decode(buf: &[u8]) -> Message {
    match std::str::from_utf8(buf) {
        Ok(data) => match ron::de::from_str(data) {
            Ok(msg) => msg,
            Err(e) => panic!("{}", e),
        },
        Err(e) => panic!("{}", e),
    }
}
fn handle_buf(buf: &[u8], conn: TcpStream) {
    let msg = decode(buf);
    println!("{:?}", msg);
    let mut subscribers = SUBSCRIBERS.lock().unwrap();
    match msg.kind {
        MessageType::Publish => {
            if msg.data == None {
                panic!("missing data in publish message")
            } else {
                let subscribed_connections = subscribers.get(&msg.channel);
                match subscribed_connections {
                    Some(connections) => {
                        for conn in connections {
                            let mut writer = BufWriter::new(conn);
                            let str_data =
                                ron::ser::to_string(&msg.data.as_ref().unwrap()).unwrap();
                            let buf = str_data.as_bytes();
                            let _ = writer.write_all(buf).unwrap();
                            writer.flush();
                        }
                    }
                    None => {
                        println!("no connections");
                    }
                }
            }
        }
        MessageType::Subscribe => {
            if let std::collections::hash_map::Entry::Vacant(e) =
                subscribers.entry(msg.channel.clone())
            {
                e.insert(Vec::from([conn]));
            } else {
                subscribers.get_mut(&msg.channel).unwrap().push(conn);
            }
        }
        MessageType::Unsubscribe => {
            let mut subscribers = SUBSCRIBERS.lock().unwrap();
            if subscribers.contains_key(&msg.channel) {
                let channel_subs = subscribers.get_mut(&msg.channel).unwrap();
                let idx = channel_subs
                    .iter()
                    .position(|x| x.local_addr().unwrap() == conn.local_addr().unwrap());
                channel_subs.remove(idx.unwrap());
            }
        }
    }
}
