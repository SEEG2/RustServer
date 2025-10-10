use crate::logger::string::Logger;
use crate::network::packet::{KeepAlivePacket, Packet, TestPacket};
use lazy_static::lazy_static;
use std::io::{Error, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering::{Acquire, Relaxed};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::util::number::random_non_zero_u32;

static SERVER_STARTED: AtomicBool = AtomicBool::new(false);
lazy_static! {
        static ref CONNECTIONS: Mutex<Vec<Connection>> = Mutex::new(Vec::new());
}

pub fn start(address: &str, port: u16) -> JoinHandle<()>{
    let address = address.to_string();

    thread::spawn(move || {
        let address_and_port_combined = &format!("{address}:{port}");
        let listener = match TcpListener::bind(address_and_port_combined) {
            Ok(s) => s,
            Err(e) => {
                format!("Server failed to bind TCP listener to {address_and_port_combined}: {e}").error();
                return
            }
        };
        SERVER_STARTED.store(true, Ordering::SeqCst);
        
        for stream in listener.incoming() {
            CONNECTIONS.lock().unwrap().push(
                 match Connection::new(match stream {
                    Ok(s) => s,
                    Err(e) => {
                        format!("Incoming server stream is invalid: {e}").error();
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
    ip: SocketAddr,
    should_shutdown: Arc<AtomicBool>,
    expected_keep_alive: Arc<AtomicU32>
}

impl Connection { // TODO consider finding a better solution for this
    const BUFFER_SIZE_BYTE: usize = 32;

    pub fn new(stream: TcpStream) -> Result<Self, Error> {
        let ip = match stream.peer_addr() {
            Ok(s) => s,
            Err(e) => {
                format!("Failed to read socket address: {e}").error();
                return Err(e) 
            }
        };
        
        match stream.set_read_timeout(Some(Duration::from_secs(120))) {
            Ok(s) => s,
            Err(e) => {
                format!("Failed to set read timeout for {ip}: {e}").error();
                return Err(e);
            }
        }

        match stream.set_write_timeout(Some(Duration::from_secs(120))) {
            Ok(s) => s,
            Err(e) => {
                format!("Failed to set write timeout for {ip}: {e}").error();
                return Err(e);
            }
        }

        let mut read_stream = match stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                format!("Failed to clone stream for {ip}: {e}").error();
                return Err(e);
            }
        };

        let keep_alive_stream = match stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                format!("Failed to clone stream for {ip}: {e}").error();
                return Err(e);
            }
        };
        
        let connection = Connection { stream, ip, should_shutdown: Arc::new(AtomicBool::new(false)), expected_keep_alive: Arc::new(AtomicU32::new(0)) };

        let read_should_shutdown = Arc::clone(&connection.should_shutdown);
        let keep_alive_should_shutdown = Arc::clone(&connection.should_shutdown);
        let read_expected_keep_alive = Arc::clone(&connection.expected_keep_alive);
        let keep_alive_expected_keep_alive = Arc::clone(&connection.expected_keep_alive);
        
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

                if n == 0 || read_should_shutdown.load(Acquire) {
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

                if boxed.len() >= 2 {
                    let id = u16::from_le_bytes([boxed[0], boxed[1]]);
                    let content: Box<[u8]> = Box::from(&boxed[2..]);
                    
                    match id { //TODO error handling here
                        TestPacket::PACKET_ID => {  
                            format!("{}", &*TestPacket::interpret(content).unwrap()).success();
                        }
                        KeepAlivePacket::PACKET_ID => { 
                            let number = KeepAlivePacket::interpret(content).unwrap();
                            
                            if number == read_expected_keep_alive.load(Acquire) {
                                format!("{ip} completed keep-alive check").success();
                                read_expected_keep_alive.store(0, Relaxed)
                            } else {
                                format!("{ip} failed keep-alive check").warning();
                                break
                                // TODO shutdown here
                            }
                        }
                        _ => {format!("Packet ID received by {ip} does not match any packet").warning();}
                    }
                }    
            }
        });

       
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(4));
                
                if keep_alive_should_shutdown.load(Acquire) { 
                    break;
                }
                
                if keep_alive_expected_keep_alive.load(Acquire) != 0 {
                    format!("{ip} did not respond to keep-alive").warning();
                    //TODO shutdown here
                    break
                }
                
                let random = random_non_zero_u32();
                match KeepAlivePacket::create(random) {
                    Ok(s) => {
                        keep_alive_expected_keep_alive.store(random, Relaxed);
                        write_to_stream(&keep_alive_stream, ip, &*s)
                    },
                    Err(e) => {
                        format!("Failed to create keep alive package for {ip}: {e}").error();
                        return 
                    }
                };
                
            }
        });

        Ok(connection)
    }
    
    pub fn stream(self) -> TcpStream {
        self.stream
    }

    fn shutdown(&mut self) {
        self.should_shutdown.store(true, Ordering::SeqCst);
        let ip = self.ip.to_string();
        
        match self.stream.shutdown(Shutdown::Both) {
            Ok(_s) => {
                format!("Connection to {ip} was shutdown").info();
            },
            Err(e) => {
                format!("Failed to shutdown connection to {ip}: {e}").error();
            }
        }
    }
}

pub fn write_to_stream(mut stream: &TcpStream, ip: SocketAddr, data: &[u8]) {
    match stream.write(&data) {
        Ok(u) => u,
        Err(e) => {
            format!("Failed to write to stream to {ip}: {e}").error();
            return;
        }
    };
}