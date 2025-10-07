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
        let listener = TcpListener::bind(address_and_port_combined)
            .expect(&format!("Server failed to bind TCP listener to {}", address_and_port_combined));
        SERVER_STARTED.store(true, Ordering::SeqCst);

        for stream in listener.incoming() {
            new_connection(stream.expect("Incoming server stream is invalid"));
        }
    })
}

const BUFFER_SIZE_BYTE: usize = 32;
fn new_connection(mut stream: TcpStream) {
    thread::spawn(move || {
        loop {
            let mut buf = [0; 8 * BUFFER_SIZE_BYTE];
            let n = stream.read(&mut buf).expect("Server failed to read from stream");

            if n == 0 {
                format!("Connection to {} was closed", stream.peer_addr().expect("Failed to read address from stream")).error();
                break;
            }

            let boxed = buf[..n].to_vec().into_boxed_slice();

            format!("{}", &*TestPacket::interpret(Box::from(&boxed[2..])).unwrap()).info();
        }
    });
}