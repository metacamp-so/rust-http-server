use super::method::Method;
use std::convert::TryFrom;
pub struct Request {
  path: String,
  query_string: Option<String>, // should be optional not mandatory
  method: Method, // represent as enum we introduced common methods
}

impl TryFrom<&[u8]> for Request { // we are taking in a ref byte array
  type Error = String;

  // GET /search?name=abc&sort=1 HTTP/1.1
  fn try_from(value: &[u8]) -> Result <Self, Self::Error> {
    unimplemented!()
  }
}