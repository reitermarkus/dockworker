use serde_aux::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runtime {
  pub path: String,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub runtime_args: Vec<String>,
}
