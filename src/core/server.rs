use std::fmt::format;
use crate::network::packet::{Packet, TestPacket};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use crate::logger::string::Logger;

static SERVER_STARTED: AtomicBool = AtomicBool::new(false);


pub fn start(address: &str, port: u16) -> JoinHandle<()>{
    let address = address.to_string();

    thread::spawn(move || {
        let address_and_port_combined = &format!("{}:{}", address, port);
        let listener = match TcpListener::bind(address_and_port_combined) {
            Ok(s) => s,
            Err(e) => {
                format!("Server failed to bind TCP listener to {}: {}", address_and_port_combined, e).error();
                return
            }
        };
        SERVER_STARTED.store(true, Ordering::SeqCst);
        
        for stream in listener.incoming() {
            new_connection(match stream {
                Ok(s) => s,
                Err(e) => {
                    format!("Incoming server stream is invalid: {}", e).error();
                    return
                }
            });
        }
    })
}

const BUFFER_SIZE_BYTE: usize = 32;
fn new_connection(mut stream: TcpStream) {
    thread::spawn(move || {
        loop {
            let mut buf = [0; 8 * BUFFER_SIZE_BYTE];
            let n = match stream.read(&mut buf) {
                Ok(s) => s,
                Err(e) => {
                    format!("Server failed to read from stream: {}", e).error();
                    break
                }
            };
            
            if n == 0 {
                format!("Connection to {} was closed", match stream.peer_addr() {
                    Ok(s) => s,
                    Err(e) => {
                        format!("Failed to read address from stream: {}", e).error();
                        break
                    }
                    
                }).info();
                break;
            }

            let boxed = buf[..n].to_vec().into_boxed_slice();

            format!("{}", &*TestPacket::interpret(Box::from(&boxed[2..])).unwrap()).info();
        }
    });
}