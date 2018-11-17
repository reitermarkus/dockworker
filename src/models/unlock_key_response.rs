#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnlockKeyResponse {
  pub unlock_key: String,
}
