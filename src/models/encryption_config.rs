#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EncryptionConfig {
  pub auto_lock_managers: bool,
}
