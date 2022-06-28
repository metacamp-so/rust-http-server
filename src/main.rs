use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;
fn main() {
    let server = Server::new("127.0.0.1:8080".to_owned()); // Server is a struct, new is an associated function
    server.run(WebsiteHandler);
}
