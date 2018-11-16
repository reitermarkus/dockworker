use models::{ContainerSpec, Driver, Placement, ServiceNetwork, TaskSpecRestartPolicy};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskSpec {
  // pub plugin_spec: PluginSpec,
  pub container_spec: ContainerSpec,
  // pub network_attachment_spec: NetworkAttachmentSpec,
  // pub resources: Resources,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub restart_policy: Option<TaskSpecRestartPolicy>,
  pub placement: Placement,
  pub force_update: u64,
  pub runtime: String,
  pub networks: Vec<ServiceNetwork>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub log_driver: Option<Driver>,
}
