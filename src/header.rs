use std::fmt;
use hyper::header::{Header, HeaderFormat};
use hyper::error::Result;
use hyper::Error;
use base64::{self, STANDARD};

#[derive(Debug, Clone)]
pub struct XRegistryAuth {
  body: String,
}

impl XRegistryAuth {
  pub fn new<S: Into<String>>(body: S) -> Self {
    Self { body: body.into() }
  }
}

impl Header for XRegistryAuth {
  fn header_name() -> &'static str {
    "X-Registry-Auth"
  }

  fn parse_header(raw: &[Vec<u8>]) -> Result<Self> {
    if raw.len() != 1 {
      return Err(Error::Header);
    }

    base64::decode_config(&raw[0], STANDARD)
      .map_err(|_| Error::Header)
      .and_then(|vec| String::from_utf8(vec).map_err(|_| Error::Header))
      .map(Self::new)
  }
}

impl HeaderFormat for XRegistryAuth {
  fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let b64 = base64::encode_config(self.body.as_bytes(), STANDARD);
    debug!("{}: {}", Self::header_name(), b64);
    write!(f, "{}", b64)
  }
}
