use std::io;

use hyper;
use serde_json;
use url;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "Connection Error")]
  Connection(#[cause] hyper::Error),

  #[fail(display = "API Error")]
  Url(#[cause] hyper::error::ParseError),

  #[fail(display = "IO Error")]
  Io(#[cause] io::Error),

  #[fail(display = "JSON Error")]
  Json(#[cause] serde_json::error::Error),

  #[fail(display = "Docker API Error: {}", message)]
  API { message: String },

  #[fail(display = "Could not connect to Docker at '{}'.", _0)]
  CouldNotConnect(String),

  #[fail(display = "Could not find DOCKER_CERT_PATH.")]
  NoCertPath,

  #[fail(display = "SSL support was disabled at compile time.")]
  SslDisabled,

  #[fail(display = "Could not connect to Docker at '{}' via SSL.", _0)]
  SslError(String),

  #[fail(display = "Do not know how to connect to Docker at '{}'.", _0)]
  UnsupportedScheme(String),

  #[fail(display = "unknown error: {}", _0)]
  Unknown(String),
}

impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    Error::Json(e)
  }
}

impl From<hyper::Error> for Error {
  fn from(e: hyper::Error) -> Self {
    Error::Connection(e)
  }
}

impl From<url::ParseError> for Error {
  fn from(e: url::ParseError) -> Self {
    Error::Url(e)
  }
}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Error::Io(e)
  }
}
