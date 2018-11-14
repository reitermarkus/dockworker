#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Top {
  pub titles: Vec<String>,
  pub processes: Vec<Vec<String>>,
}
