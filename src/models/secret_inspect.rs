use models::SecretInspectSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretInspect {
  #[serde(rename = "ID")]
  pub id: String,
  pub created_at: String,
  pub updated_at: String,
  pub spec: SecretInspectSpec
}