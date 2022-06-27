pub struct Server {
  addr: String,
}

impl Server {
  // functionality for Server "method"
  pub fn new(addr: String) -> Server {
      Server { addr: addr }
  }

  pub fn run(self) { // follows ownership rule, use & if don't want to take ownership
      println!("Listening on {}", self.addr)
  }
}