use serde_aux::prelude::*;

use std::collections::HashMap as Map;
use std::path::PathBuf;

use models::{Commit, Plugins, Runtime, SwarmInfo};

#[derive(Debug, Clone, Deserialize)]
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

#[cfg(test)]
mod tests {
  use super::*;

  use serde_json;

  #[test]
  fn deserialize() {
    serde_json::from_str::<SystemInfo>(&r#"
      {
        "ID": "ZVJI:3MQN:LXLI:CS7E:5Y2W:2N4M:XLEO:U4FJ:EXUU:N6XE:IO4F:3U4E",
        "Containers": 4,
        "ContainersRunning": 0,
        "ContainersPaused": 0,
        "ContainersStopped": 4,
        "Images": 17,
        "Driver": "overlay2",
        "DriverStatus": [
          ["Backing Filesystem", "extfs"],
          ["Supports d_type", "true"],
          ["Native Overlay Diff", "true"]
        ],
        "SystemStatus": null,
        "Plugins": {
          "Volume": ["local"],
          "Network": ["bridge", "host", "ipvlan", "macvlan", "null", "overlay"],
          "Authorization": null,
          "Log": [
            "awslogs",
            "fluentd",
            "gcplogs",
            "gelf",
            "journald",
            "json-file",
            "logentries",
            "splunk",
            "syslog"
          ]
        },
        "MemoryLimit": true,
        "SwapLimit": true,
        "KernelMemory": true,
        "CpuCfsPeriod": true,
        "CpuCfsQuota": true,
        "CPUShares": true,
        "CPUSet": true,
        "IPv4Forwarding": true,
        "BridgeNfIptables": true,
        "BridgeNfIp6tables": true,
        "Debug": true,
        "NFd": 40,
        "OomKillDisable": true,
        "NGoroutines": 165,
        "SystemTime": "2018-11-13T10:04:29.645970285Z",
        "LoggingDriver": "json-file",
        "CgroupDriver": "cgroupfs",
        "NEventsListener": 2,
        "KernelVersion": "4.9.93-linuxkit-aufs",
        "OperatingSystem": "Docker for Mac",
        "OSType": "linux",
        "Architecture": "x86_64",
        "IndexServerAddress": "https://index.docker.io/v1/",
        "RegistryConfig": {
          "AllowNondistributableArtifactsCIDRs": [],
          "AllowNondistributableArtifactsHostnames": [],
          "InsecureRegistryCIDRs": ["127.0.0.0/8"],
          "IndexConfigs": {
            "docker.io": {
              "Name": "docker.io",
              "Mirrors": [],
              "Secure": true,
              "Official": true
            }
          },
          "Mirrors": []
        },
        "NCPU": 4,
        "MemTotal": 2095763456,
        "GenericResources": null,
        "DockerRootDir": "/var/lib/docker",
        "HttpProxy": "gateway.docker.internal:3128",
        "HttpsProxy": "gateway.docker.internal:3129",
        "NoProxy": "",
        "Name": "linuxkit-025000000001",
        "Labels": [],
        "ExperimentalBuild": true,
        "ServerVersion": "18.06.1-ce",
        "ClusterStore": "",
        "ClusterAdvertise": "",
        "Runtimes": {
          "runc": {
            "path": "docker-runc"
          }
        },
        "DefaultRuntime": "runc",
        "Swarm": {
          "NodeID": "ovq7ih4ls99hf09horz93kc9q",
          "NodeAddr": "192.168.65.3",
          "LocalNodeState": "active",
          "ControlAvailable": true,
          "Error": "",
          "RemoteManagers": [
            {
              "NodeID": "ovq7ih4ls99hf09horz93kc9q",
              "Addr": "192.168.65.3:2377"
            }
          ],
          "Nodes": 1,
          "Managers": 1,
          "Cluster": {
            "ID": "luu4hbkk0c9gjwdbclzks7nrq",
            "Version": {
              "Index": 10
            },
            "CreatedAt": "2018-11-13T10:04:24.39833508Z",
            "UpdatedAt": "2018-11-13T10:04:25.016267841Z",
            "Spec": {
              "Name": "default",
              "Labels": {},
              "Orchestration": {
                "TaskHistoryRetentionLimit": 5
              },
              "Raft": {
                "SnapshotInterval": 10000,
                "KeepOldSnapshots": 0,
                "LogEntriesForSlowFollowers": 500,
                "ElectionTick": 10,
                "HeartbeatTick": 1
              },
              "Dispatcher": {
                "HeartbeatPeriod": 5000000000
              },
              "CAConfig": {
                "NodeCertExpiry": 7776000000000000
              },
              "TaskDefaults": {},
              "EncryptionConfig": {
                "AutoLockManagers": false
              }
            },
            "TLSInfo": {
              "TrustRoot": "-----BEGIN CERTIFICATE-----\nMIIBajCCAQ+gAwIBAgITJpZgNZ98hlQl7q7ENWDkTpOE5jAKBggqhkjOPQQDAjAT\nMREwDwYDVQQDEwhzd2FybS1jYTAeFw0xODExMTMwOTU5MDBaFw0zODExMDgwOTU5\nMDBaMBMxETAPBgNVBAMTCHN3YXJtLWNhMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcD\nQgAE7ieT5otOQQmEgMopQa49UDjyQvbqu15LxdNHyeh6PELSAgoGtS8TXfJpcsaR\nM0lk3cb9oAOBcjGsqgN6R3OqjaNCMEAwDgYDVR0PAQH/BAQDAgEGMA8GA1UdEwEB\n/wQFMAMBAf8wHQYDVR0OBBYEFE0I/FQsox/otZ37tCWIJFF67wyVMAoGCCqGSM49\nBAMCA0kAMEYCIQC/y4sD4qcHEfuWVRDrGvA1jwk0YfkZ2Qkr45MLh4t/RQIhAJqR\n3+TgVV/+2G4YyDUVIrH9ssL1RBm+l/aqLWSgMR2r\n-----END CERTIFICATE-----\n",
              "CertIssuerSubject": "MBMxETAPBgNVBAMTCHN3YXJtLWNh",
              "CertIssuerPublicKey": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE7ieT5otOQQmEgMopQa49UDjyQvbqu15LxdNHyeh6PELSAgoGtS8TXfJpcsaRM0lk3cb9oAOBcjGsqgN6R3OqjQ=="
            },
            "RootRotationInProgress": false
          }
        },
        "LiveRestoreEnabled": false,
        "Isolation": "",
        "InitBinary": "docker-init",
        "ContainerdCommit": {
          "ID": "468a545b9edcd5932818eb9de8e72413e616e86e",
          "Expected": "468a545b9edcd5932818eb9de8e72413e616e86e"
        },
        "RuncCommit": {
          "ID": "69663f0bd4b60df09991c08812a60108003fa340",
          "Expected": "69663f0bd4b60df09991c08812a60108003fa340"
        },
        "InitCommit": {
          "ID": "fec3683",
          "Expected": "fec3683"
        },
        "SecurityOptions": ["name=seccomp,profile=default"]
      }
    "#).unwrap();
  }
}
