use models::{IdentityToken, UserPassword};
use header::XRegistryAuth;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Credential {
  Token(IdentityToken),
  Password(UserPassword),
}

impl Credential {
  pub fn with_token(token: IdentityToken) -> Self {
    Credential::Token(token)
  }

  pub fn with_password(password: UserPassword) -> Self {
    Credential::Password(password)
  }
}

impl From<Credential> for XRegistryAuth {
  fn from(credential: Credential) -> Self {
    XRegistryAuth::new(serde_json::to_string(&credential).unwrap())
  }
}
