fn main() {

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

// Impl
impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr 
        }
    }
    
    fn run(self) {
        println!("Listening on {}", self.addr)
    }
    
}

// Request struct, no body or headers.
struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    OPTIONS,
    DELETE,
    CONNECT,
    TRACE,
    PATCH,
    HEAD,
}