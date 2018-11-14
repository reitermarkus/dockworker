use std::fmt;
use std::path::PathBuf;

use serde::de::{self, DeserializeOwned, Deserializer, Visitor};
use serde::{Deserialize};

use swarm::{Spec, TLSInfo};

struct NumToBoolVisitor;

impl<'de> Visitor<'de> for NumToBoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("0 or 1 or true or false")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value != 0)
    }
}

fn num_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(NumToBoolVisitor)
}

fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned + Default,
{
    let opt: Option<T> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Plugins {
  #[serde(default, deserialize_with = "null_to_default")]
  pub volume: Vec<String>,
  #[serde(default, deserialize_with = "null_to_default")]
  pub network: Vec<String>,
  #[serde(default, deserialize_with = "null_to_default")]
  pub authorization: Vec<String>,
  #[serde(default, deserialize_with = "null_to_default")]
  pub log: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClusterVersion {
  pub index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cluster {
  #[serde(rename = "ID")]
  pub id: String,
  pub version: ClusterVersion,
  pub created_at: String,
  pub updated_at: String,
  pub spec: Spec,
  #[serde(rename = "TLSInfo")]
  pub tls_info: TLSInfo,
  pub root_rotation_in_progress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Swarm {
  #[serde(rename = "NodeID")]
  pub node_id: String,
  pub node_addr: String,
  pub local_node_state: String,
  pub control_available: bool,
  pub error: String,
  // #[serde(default = "Vec::default", deserialize_with = "null_to_default")]
  // pub remote_managers: Vec<RemoteManager>,
  #[serde(default, deserialize_with = "null_to_default")]
  pub nodes: u64,
  #[serde(default, deserialize_with = "null_to_default")]
  pub managers: u64,
  pub cluster: Option<Cluster>,
}

/// response of /info
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
    #[serde(default, deserialize_with = "null_to_default")]
    pub system_status: String,
    pub plugins: Plugins,
    #[serde(deserialize_with = "num_to_bool")]
    pub memory_limit: bool,
    #[serde(deserialize_with = "num_to_bool")]
    pub swap_limit: bool,
    pub kernel_memory: bool,
    pub cpu_cfs_period: bool,
    pub cpu_cfs_quota: bool,
    #[serde(rename = "CPUShares")]
    pub cpu_shares: bool,
    #[serde(rename = "CPUSet")]
    pub cpu_set: bool,
    #[serde(deserialize_with = "num_to_bool", rename = "IPv4Forwarding")]
    pub ipv4_forwarding: bool,
    pub bridge_nf_iptables: bool,
    pub bridge_nf_ip6tables: bool,
    #[serde(deserialize_with = "num_to_bool")]
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
    #[serde(default = "Vec::default", deserialize_with = "null_to_default")]
    pub generic_resources: Vec<String>,
    pub docker_root_dir: PathBuf,
    pub mem_total: u64,
    pub http_proxy: String,
    pub https_proxy: String,
    pub no_proxy: String,
    pub name: String,
    #[serde(default = "Vec::default", deserialize_with = "null_to_default")]
    pub labels: Vec<String>,
    pub experimental_build: bool,
    pub server_version: String,
    pub cluster_store: String,
    pub cluster_advertise: String,
    // pub runtimes: HashMap<String, Runtime>,
    pub default_runtime: String,
    pub swarm: Swarm,
    pub live_restore_enabled: bool,
    pub isolation: String,
    pub init_binary: String,
    // pub container_commit: Commit,
    // pub run_commit: Commit,
    // pub init_commit: Commit,
    pub security_options: Vec<String>,
}

/// Type of the response of `/auth` api
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthToken {
    status: String,
    identity_token: String,
}

impl AuthToken {
    #[allow(dead_code)]
    pub fn token(&self) -> String {
        self.identity_token.clone()
    }
}
