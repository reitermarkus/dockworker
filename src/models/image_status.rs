use serde_aux::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageStatus {
  status: String,
  #[serde(default, deserialize_with = "deserialize_default_from_empty_object")]
  progress_detail: Option<ProgressDetail>,
  progress: Option<String>,
  id: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressDetail {
  pub current: u64,
  pub total: u64,
}
