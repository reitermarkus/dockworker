use serde_aux::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Plugins {
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub volume: Vec<String>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub network: Vec<String>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub authorization: Vec<String>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub log: Vec<String>,
}
