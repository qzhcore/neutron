use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct HttpServer {
    addr: String,
}

impl HttpServer {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        let _ = Self::handle_connection(stream);
                    });
                }
                Err(_) => {}
            }
        }
        Ok(())
    }

    fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\nContent-Type: text/plain\r\n\r\nHello, Neutron";
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
