#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
  // Name (optional)
  // Driver (optional)
  pub source: String,
  pub destination: String,
  pub mode: String,
  #[serde(rename = "RW")]
  pub rw: bool,
  pub propagation: String,
}
