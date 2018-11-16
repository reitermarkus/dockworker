use std::collections::HashMap as Map;

use serde_aux::prelude::*;

use models::{EndpointSpec, Mode, RollbackConfig, ServiceNetwork, UpdateConfig, TaskSpec};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceSpec {
  pub name: String,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub labels: Map<String, String>,
  pub task_template: TaskSpec,
  pub mode: Mode,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub update_config: Option<UpdateConfig>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rollback_config: Option<RollbackConfig>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub networks: Vec<ServiceNetwork>,
  pub endpoint_spec: EndpointSpec,
}
