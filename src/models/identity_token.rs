use header::XRegistryAuth;
use models::AuthResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityToken {
  identity_token: String,
}

impl From<String> for IdentityToken {
  fn from(identity_token: String) -> Self {
    Self { identity_token }
  }
}

impl From<AuthResponse> for IdentityToken {
  fn from(auth_response: AuthResponse) -> Self {
    auth_response.identity_token.into()
  }
}

impl From<IdentityToken> for String {
  fn from(identity_token: IdentityToken) -> String {
    identity_token.identity_token.clone()
  }
}

impl From<IdentityToken> for XRegistryAuth {
  fn from(identity_token: IdentityToken) -> Self {
    Self::new(identity_token)
  }
}
