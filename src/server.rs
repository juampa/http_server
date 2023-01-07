use std::io::{Read, Write};
use std::net::TcpListener;
use std::convert::TryFrom;
use std::convert::TryInto;

use crate::http:: { Request, Response, StatusCode, ParseError };

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response; 
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request {}", e);
        Response::new(StatusCode::BadRequest,None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        // unwrap convierte el posible error en no recuperable.
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {

                    // Podría ser un error si leemos más de 1024.
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request {} ", String::from_utf8_lossy(&buffer));
                
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }    
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        } 
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                            
                        }
                    }                    
                }
                Err(e) => println!("Failed to established connection: {}", e),
            }
        }
    }
}
