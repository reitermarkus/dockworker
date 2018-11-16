use failure::Fail;
use hyper::Response;
use hyper::HeaderMap;
use hyper::Body;
use std::{
  path::Path,
  result::Result,
};

pub trait HttpClient {
  type Err: Fail + Send + 'static;

  fn get<S: AsRef<str>>(&self, headers: &HeaderMap, path: S)
    -> Result<Response<Body>, Self::Err>;

  fn post<S: AsRef<str>>(&self, headers: &HeaderMap, path: S, body: &str)
    -> Result<Response<Body>, Self::Err>;

  fn delete<S: AsRef<str>>(&self, headers: &HeaderMap, path: S)
    -> Result<Response<Body>, Self::Err>;

  fn post_file<S: AsRef<str>>(&self, headers: &HeaderMap, path: S, file: &Path)
    -> Result<Response<Body>, Self::Err>;

  fn put_file<S: AsRef<str>>(&self, headers: &HeaderMap, path: S, file: &Path)
    -> Result<Response<Body>, Self::Err>;
}
