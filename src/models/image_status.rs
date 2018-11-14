#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageStatus {
  pub status: Option<String>,
  pub error: Option<String>,
}
