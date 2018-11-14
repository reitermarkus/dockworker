#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthToken {
  status: String,
  identity_token: String,
}

impl AuthToken {
  #[allow(dead_code)]
  pub fn token(&self) -> String {
    self.identity_token.clone()
  }
}
