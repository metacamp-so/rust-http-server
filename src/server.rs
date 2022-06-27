use std::net::TcpListener;
use std::io::Read;
pub struct Server {
  addr: String,
}

impl Server {
  // functionality for Server "method"
  pub fn new(addr: String) -> Server {
      Server { addr: addr }
  }

  pub fn run(self) { // follows ownership rule, use & if don't want to take ownership
      println!("Listening on {}", self.addr);

      let listener = TcpListener::bind(&self.addr).unwrap(); // we unwrap to convert into unrecoverable error

      loop { // loops infinitely
        match listener.accept() {
          Ok((mut stream, _)) => {
            let mut buffer = [0; 1024]; // mutable buffer for .read to write 1024 is ok for our simple server
            match stream.read(&mut buffer) {
              Ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buffer))
              },
              Err(e) => println!("Failed to read from connection: {}", e),
            }
          },
          Err(e) => println!("Failed to establish a connection: {}", e),
        }
      }
  }
}