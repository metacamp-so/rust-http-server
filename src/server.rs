use std::net::TcpListener;
use std::convert::TryFrom;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode, ParseError};
pub struct Server {
  addr: String,
}

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, e: &ParseError) -> Response {
    println!("Failed to parse request: {}", e);
    Response::new(StatusCode::BadRequest, None)
  }
}

impl Server {
  // functionality for Server "method"
  pub fn new(addr: String) -> Server {
      Server { addr: addr }
  }

  pub fn run<T:Handler>(self, mut handler: T) { // follows ownership rule, use & if don't want to take ownership
      println!("Listening on {}", self.addr);

      let listener = TcpListener::bind(&self.addr).unwrap(); // we unwrap to convert into unrecoverable error

      loop { // loops infinitely
        match listener.accept() {
          Ok((mut stream, _)) => {
            let mut buffer = [0; 1024]; // mutable buffer for .read to write 1024 is ok for our simple server
            match stream.read(&mut buffer) {
              Ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                let response = match Request::try_from(&buffer[..]) {//pass in a byte slice because trait is expecting
                  Ok(request) => handler.handle_request(&request),
                  Err(e) => handler.handle_bad_request(&e),
                };
                
                if let Err(e) = response.send(&mut stream) {
                  println!("Failed to send response: {}", e);
                }
              },
              Err(e) => println!("Failed to read from connection: {}", e),
            }
          },
          Err(e) => println!("Failed to establish a connection: {}", e),
        }
      }
  }
}