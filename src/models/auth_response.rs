use models::IdentityToken;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthResponse {
  pub status: String,
  #[serde(flatten)]
  pub identity_token: IdentityToken,
}
