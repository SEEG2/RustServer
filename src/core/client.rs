use std::io::Write;
use std::net::TcpStream;
use std::ops::Add;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};
use crate::logger::string::Logger;
use crate::network::packet::{Packet, TestPacket};

pub fn run_new(port: u16, time_sec: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        let address_and_port_combined = &format!("localhost:{}", port);
        let end_time = SystemTime::now().add(Duration::from_secs(time_sec));
        let mut stream = match TcpStream::connect(address_and_port_combined) {
            Ok(s) => s,
            Err(e) => {
                format!("Client failed to connect to {}: {}", address_and_port_combined, e).error();
                return
            }
        };
        
        loop {
            let packet = match TestPacket::create("H".to_string()) {
                Ok(p) => p,
                Err(e) => {
                    format!("Failed to create TestPacket: {}", e).error();
                    break;
                }
            };
            match stream.write(&*packet) {
                Ok(_) => {},
                Err(e) => {
                    format!("Test Client failed to write to stream: {}", e).error();
                    break;
                }
            }        
            
            if SystemTime::now().gt(&end_time) {
                break
            }
            thread::sleep(Duration::from_secs(1));
        }
    })
}