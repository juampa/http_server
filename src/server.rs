use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
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
                        } 
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }                    
                }
                Err(e) => println!("Failed to established connection: {}", e),
            }
        }
    }
}
