use super::method::Method;
pub struct Request {
  path: String,
  query_string: Option<String>, // should be optional not mandatory
  method: Method, // represent as enum we introduced common methods
}