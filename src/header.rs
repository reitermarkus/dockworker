use std::fmt;

use base64;
use bytes::Bytes;
use hyper::header::{HeaderName, HeaderValue};

#[derive(Debug, Clone)]
pub struct XRegistryAuth {
  body: String,
}

pub const X_REGISTRY_AUTH: HeaderName = HeaderName::from_static("X-Registry-Auth");

impl XRegistryAuth {
  pub fn new<S: Into<String>>(body: S) -> Self {
    Self { body: body.into() }
  }
}

impl From<HeaderValue> for XRegistryAuth {
  fn from(header_value: HeaderValue) -> Self {
    let data = base64::decode(&header_value.as_bytes()).ok()
      .and_then(|s| String::from_utf8(s).ok())
      .unwrap_or(String::new());

    Self::new(data)
  }
}

impl From<XRegistryAuth> for HeaderValue {
  fn from(auth: XRegistryAuth) -> Self {
    Self {
      inner: Bytes::from(base64::encode(&auth.body).as_bytes()),
      is_sensitive: true,
    }
  }
}
