use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

pub struct Request {
  path: String,
  query_string: Option<String>, // should be optional not mandatory
  method: Method, // represent as enum we introduced common methods
}

impl TryFrom<&[u8]> for Request { // we are taking in a ref byte array
  type Error = ParseError;

  // GET /search?name=abc&sort=1 HTTP/1.1
  fn try_from(value: &[u8]) -> Result <Self, Self::Error> {

    let request = str::from_utf8(value)?;
    unimplemented!()
  }
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self{
    Self::InvalidEncoding
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      ParseError::InvalidRequest => "Invalid Request",
      ParseError::InvalidEncoding => "Invalid Encoding",
      ParseError::InvalidProtocol => "Invalid Protocol",
      ParseError::InvalidMethod => "Invalid Method",
    }
  }
}

impl Error for ParseError {

}