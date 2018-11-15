use serde_aux::prelude::*;

use std::collections::HashMap as Map;
use std::path::PathBuf;

use models::{Commit, Plugins, Runtime, SwarmInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemInfo {
  #[serde(rename = "ID")]
  pub id: String,
  pub containers: u64,
  pub containers_running: u64,
  pub containers_paused: u64,
  pub containers_stopped: u64,
  pub images: u64,
  pub driver: String,
  pub driver_status: Vec<(String, String)>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub system_status: String,
  pub plugins: Plugins,
  #[serde(deserialize_with = "deserialize_bool_from_anything")]
  pub memory_limit: bool,
  #[serde(deserialize_with = "deserialize_bool_from_anything")]
  pub swap_limit: bool,
  pub kernel_memory: bool,
  pub cpu_cfs_period: bool,
  pub cpu_cfs_quota: bool,
  #[serde(rename = "CPUShares")]
  pub cpu_shares: bool,
  #[serde(rename = "CPUSet")]
  pub cpu_set: bool,
  #[serde(deserialize_with = "deserialize_bool_from_anything", rename = "IPv4Forwarding")]
  pub ipv4_forwarding: bool,
  pub bridge_nf_iptables: bool,
  pub bridge_nf_ip6tables: bool,
  #[serde(deserialize_with = "deserialize_bool_from_anything")]
  pub debug: bool,
  pub n_fd: u64,
  pub oom_kill_disable: bool,
  pub n_goroutines: u64,
  pub system_time: String,
  pub logging_driver: String,
  pub cgroup_driver: String,
  pub n_events_listener: u64,
  pub kernel_version: String,
  pub operating_system: String,
  #[serde(rename = "OSType")]
  pub os_type: String,
  pub architecture: String,
  pub index_server_address: String,
  // pub registry_config: RegistryConfig,
  #[serde(rename = "NCPU")]
  pub n_cpu: u64,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub generic_resources: Vec<String>,
  pub docker_root_dir: PathBuf,
  pub mem_total: u64,
  pub http_proxy: String,
  pub https_proxy: String,
  pub no_proxy: String,
  pub name: String,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub labels: Vec<String>,
  pub experimental_build: bool,
  pub server_version: String,
  pub cluster_store: String,
  pub cluster_advertise: String,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub runtimes: Map<String, Runtime>,
  pub default_runtime: String,
  pub swarm: SwarmInfo,
  pub live_restore_enabled: bool,
  pub isolation: String,
  pub init_binary: String,
  pub containerd_commit: Commit,
  pub runc_commit: Commit,
  pub init_commit: Commit,
  pub security_options: Vec<String>,
}
