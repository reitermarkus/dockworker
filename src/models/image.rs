use serde_aux::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
  pub id: String,
  pub parent_id: String,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub repo_tags: Vec<String>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub repo_digests: Vec<String>,
  pub created: u64,
  pub size: i64,
  #[serde(default)]
  pub shared_size: i64,
  pub virtual_size: i64,
  #[serde(default)]
  pub containers: i64,
}
