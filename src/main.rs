mod network;
mod core;

use std::thread;
use std::time::Duration;
use crate::core::client;
use crate::core::server;

fn main() {
    let port = 34254;
    server::start("0.0.0.0", port);
    client::run_new(port, 5);
    
    thread::sleep(Duration::new(4242, 0));
}
