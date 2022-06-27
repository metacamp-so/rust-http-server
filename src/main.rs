use server::Server;
use http::Method;
use http::Request;

mod server;
mod http;
fn main() {
    let server = Server::new("127.0.0.1:8080".to_owned()); // Server is a struct, new is an associated function
    server.run();
}
