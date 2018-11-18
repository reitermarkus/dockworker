#[derive(Debug, Clone, Deserialize)]
pub struct IdResponse {
  #[serde(rename = "ID")]
  pub id: String,
}
