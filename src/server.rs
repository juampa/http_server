use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::convert::TryFrom;
use std::convert::TryInto;

use crate::http:: { Request, Response, StatusCode, ParseError };
use crate::website_handler::WebsiteHandler;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response; 
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request {}", e);
        Response::new(StatusCode::BadRequest,None)
    }
}
pub struct Server {
    addr: String,
    public_path: String
}

impl Server {
    pub fn new(addr: String, public_path: String) -> Self {
        Self { addr, public_path }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        // unwrap convierte el posible error en no recuperable.
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            
            for stream in listener.incoming() {
                let stream = stream.unwrap() ;

                thread::spawn(|| { 
                    Self::handle_connection(stream, &self.public_path);
                });
            }
        }
    }

    fn handle_connection(mut stream: TcpStream, public_path: &String) {
        let mut buffer = [0; 1024];

        let mut handler = WebsiteHandler::new(public_path.clone());

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

}
