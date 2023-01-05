
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
            listener.accept() ;
        }  

        println!("Fin") ;
    }
}
