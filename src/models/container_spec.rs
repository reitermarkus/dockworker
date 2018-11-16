use std::collections::HashMap as Map;

use serde_aux::prelude::*;

use models::{Isolation, Mount, /* Privileges */};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerSpec {
  pub image: String,
  pub labels: Map<String, String>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub command: Vec<String>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub args: Vec<String>,
  pub hostname: Option<String>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub env: Vec<String>,
  pub dir: Option<String>,
  pub user: Option<String>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub groups: Vec<String>,
  // pub privileges: Privileges,
  pub tty: Option<bool>,
  pub open_stdin: Option<bool>,
  pub read_only: Option<bool>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub mounts: Vec<Mount>,
  pub stop_signal: Option<String>,
  pub stop_grace_period: Option<i64>,
  // pub health_check: HealthConfig,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub hosts: Vec<String>,
  // #[serde(rename = "DNSConfig")]
  // pub dns_config: DNSConfig,
  // pub secrets: Vec<ContainerSecret>,
  // pub configs: Vec<ContainerConfig>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub isolation: Isolation,
  pub init: Option<bool>,
}
