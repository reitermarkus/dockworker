use failure::Fail;
use hyper::{
  client::response::Response,
  header::Headers,
};
use std::{
  path::Path,
  result::Result,
};

pub trait HttpClient {
  type Err: Fail + Send + 'static;

  fn get(&self, headers: &Headers, path: &str)
    -> Result<Response, Self::Err>;

  fn post(&self, headers: &Headers, path: &str, body: &str)
    -> Result<Response, Self::Err>;

  fn delete(&self, headers: &Headers, path: &str)
    -> Result<Response, Self::Err>;

  fn post_file(&self, headers: &Headers, path: &str, file: &Path)
    -> Result<Response, Self::Err>;

  fn put_file(&self, headers: &Headers, path: &str, file: &Path)
    -> Result<Response, Self::Err>;
}
