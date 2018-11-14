use std::{fmt, result};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ImageId {
  id: String,
}

impl From<String> for ImageId {
  fn from(id: String) -> Self {
    Self { id }
  }
}

impl fmt::Display for ImageId {
  fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
    self.id.fmt(f)
  }
}

impl ImageId {
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self { id: id.into() }
  }

  pub fn id(&self) -> &str {
    &self.id
  }
}
