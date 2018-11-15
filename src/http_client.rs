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

  fn get<S: AsRef<str>>(&self, headers: &Headers, path: S)
    -> Result<Response, Self::Err>;

  fn post<S: AsRef<str>>(&self, headers: &Headers, path: S, body: &str)
    -> Result<Response, Self::Err>;

  fn delete<S: AsRef<str>>(&self, headers: &Headers, path: S)
    -> Result<Response, Self::Err>;

  fn post_file<S: AsRef<str>>(&self, headers: &Headers, path: S, file: &Path)
    -> Result<Response, Self::Err>;

  fn put_file<S: AsRef<str>>(&self, headers: &Headers, path: S, file: &Path)
    -> Result<Response, Self::Err>;
}
