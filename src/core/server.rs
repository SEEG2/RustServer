use crate::logger::string::Logger;
use crate::network::packet::{Packet, TestPacket};
use lazy_static::lazy_static;
use std::io::{Error, Read};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

static SERVER_STARTED: AtomicBool = AtomicBool::new(false);
lazy_static! {
        static ref CONNECTIONS: Mutex<Vec<Connection>> = Mutex::new(Vec::new());
}

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
            CONNECTIONS.lock().unwrap().push(
                 match Connection::new(match stream {
                    Ok(s) => s,
                    Err(e) => {
                        format!("Incoming server stream is invalid: {}", e).error();
                        return
                    }
                }) {
                     Ok(c) => c,
                     Err(e) => {
                         "Failed to create new connection.".error();
                         continue;
                     }
                 }
            );
        }
    })
}

struct Connection {
    stream: TcpStream,
    should_shutdown: Arc<AtomicBool>
}

impl Connection {
    const BUFFER_SIZE_BYTE: usize = 32;

    pub fn new(stream: TcpStream) -> Result<Self, Error> {
        let ip = match stream.peer_addr() {
            Ok(s) => s.to_string(),
            Err(_e) => "UNKNOWN".to_string(),
        };


        match stream.set_read_timeout(Some(Duration::from_secs(120))) {
            Ok(s) => s,
            Err(e) => {
                format!("Failed to set read timeout for {}: {}", ip, e).error();
                return Err(e);
            }
        }

        match stream.set_write_timeout(Some(Duration::from_secs(120))) {
            Ok(s) => s,
            Err(e) => {
                format!("Failed to set write timeout for {}: {}", ip, e).error();
                return Err(e);
            }
        }

        let mut read_stream = match stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                format!("Failed to clone stream for {}: {}", ip, e).error();
                return Err(e);
            }
        };

        let mut write_stream = match stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                format!("Failed to clone stream for {}: {}", ip, e).error();
                return Err(e);
            }
        };

        
        
        let connection = Connection { stream, should_shutdown: Arc::new(AtomicBool::new(false)) };

        let read_should_shutdown = Arc::clone(&connection.should_shutdown);
        let write_should_shutdown = Arc::clone(&connection.should_shutdown);
        
        thread::spawn(move || {
            loop {
                let mut buf = [0; 8 * Self::BUFFER_SIZE_BYTE];
                let n = match read_stream.read(&mut buf) {
                    Ok(s) => s,
                    Err(e) => {
                        format!("Server failed to read from stream: {}", e).error();
                        break;
                    }
                };

                if n == 0 || read_should_shutdown.load(Ordering::Acquire) {
                    format!("Connection to {} was closed", match read_stream.peer_addr() {
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

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(100));

                if write_should_shutdown.load(Ordering::Acquire) {
                    break
                }
            }
        });

        Ok(connection)
    }

    pub fn stream(self) -> TcpStream {
        self.stream
    }

    fn shutdown(&mut self) {
        self.should_shutdown.store(true, Ordering::SeqCst);

        let ip = match self.stream.peer_addr() {
            Ok(s) => s.to_string(),
            Err(_e) => "UNKNOWN".to_string(),
        };

        match self.stream.shutdown(Shutdown::Both) {
            Ok(_s) => {
                format!("Connection to {} was shutdown", ip).info();
            },
            Err(e) => {
                format!("Failed to shutdown connection to {}: {}", ip, e).error();
            }
        }
    }
}