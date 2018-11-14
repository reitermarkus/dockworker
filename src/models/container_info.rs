use models::{Config, Mount, NetworkSettings, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
  pub app_armor_profile: String,
  pub args: Vec<String>,
  pub config: Config,
  pub created: String,
  pub driver: String,
  // ExecIDs
  // GraphDriver
  // HostConfig
  pub hostname_path: String,
  pub hosts_path: String,
  pub id: String,
  pub image: String,
  pub log_path: String,
  pub mount_label: String,
  pub mounts: Vec<Mount>,
  pub name: String,
  pub network_settings: NetworkSettings,
  pub path: String,
  pub process_label: String,
  pub resolv_conf_path: String,
  pub restart_count: u64,
  pub state: State,
}

impl std::fmt::Display for ContainerInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self.id)
  }
}
