fn main() {
    let server = Server::new("127.0.0.1:8080".to_owned()); // Server is a struct, new is an associated function
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    // functionality for Server "method"
    fn new(addr: String) -> Server {
        Server { addr: addr }
    }

    fn run(self) { // follows ownership rule, use & if don't want to take ownership
        println!("Listening on {}", self.addr)
    }
}

struct Request {
    path: String,
    query_string: Option<String>, // should be optional not mandatory
    method: Method, // represent as enum we introduced common methods
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/