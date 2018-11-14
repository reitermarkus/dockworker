use models::{AuthToken};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct IdentityToken {
  identitytoken: String,
}

impl IdentityToken {
  #[allow(dead_code)]
  pub fn token(&self) -> String {
    self.identitytoken.clone()
  }

  #[allow(dead_code)]
  pub fn from_auth_token(auth_token: &AuthToken) -> Self {
    Self {
      identitytoken: auth_token.token(),
    }
  }
}
