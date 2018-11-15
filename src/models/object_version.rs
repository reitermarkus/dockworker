#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
  pub index: u64,
}
