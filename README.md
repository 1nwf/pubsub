# pubsub
an implementation of the publish-subscribe messaging pattern in rust

# Usage
First of all, clone the repo: <br>
``` git clone https://github.com/nwf03/pubsub.git```
## Server
```cd pubsub/server``` and then run ``` cargo run ``` <br>
This will start a TCP server listenting on ```127.0.0.1:6379```
## Client
### Example
```rust
use client::Client;
use std::thread;

fn main() {
    let mut subscriber = Client::new("localhost:6379");
    let channel_name = String::from("channel");
    thread::spawn(|| loop {
        let mut publisher = Client::new("localhost:6379");
        thread::sleep(Duration::from_millis(500));
        let channel_name = String::from("channel");
        let data = &String::from("hello!");
        publisher.publish(&channel_name, data)
    });

    subscriber.subscribe(&channel_name);
    for msg in subscriber.recieve_message() {
        println!("{}", msg);
    }

}
```
