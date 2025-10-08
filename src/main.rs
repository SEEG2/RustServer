mod network;
mod core;
mod rich_text;
mod logger;

use std::thread;
use std::time::Duration;
use crate::core::client;
use crate::core::server;
use crate::logger::string::Logger;

fn main() {
    let port = 34254;
    server::start("0.0.0.0", port);
    format!("Server started on port {}", port).info();
    
    client::run_new(port, 5);
    thread::sleep(Duration::new(4242, 0));
}
