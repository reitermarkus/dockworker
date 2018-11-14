use serde_helpers::null_to_default;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runtime {
  pub path: String,
  #[serde(default, deserialize_with = "null_to_default")]
  pub runtime_args: Vec<String>,
}
