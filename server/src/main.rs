mod message;
use server::Server;
use std::sync::Mutex;

mod server;
use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    static ref SUBSCRIBERS: Mutex<HashMap<String, Vec<usize>>> = Mutex::new(HashMap::new());
}
fn main() {
    let server = Server::new();
    server.start()
}
