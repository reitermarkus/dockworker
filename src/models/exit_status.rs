
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExitStatus {
  status_code: i32,
}

impl ExitStatus {
  pub fn new(status_code: i32) -> Self {
    Self {
      status_code: status_code,
    }
  }

  pub fn into_inner(self) -> i32 {
    self.status_code
  }
}

impl From<i32> for ExitStatus {
  fn from(status_code: i32) -> Self {
    Self::new(status_code)
  }
}
