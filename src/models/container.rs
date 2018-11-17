use std::collections::HashMap as Map;

use models::{HostConfig, Port};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
  pub id: String,
  pub image: String,
  pub status: String,
  pub command: String,
  pub created: u64,
  pub names: Vec<String>,
  pub ports: Vec<Port>,
  pub size_rw: Option<u64>, // I guess it is optional on Mac.
  pub size_root_fs: Option<u64>,
  pub labels: Option<Map<String, String>>,
  pub host_config: HostConfig,
}

impl std::fmt::Display for Container {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self.id)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use serde_json;

  #[test]
  fn deserialize() {
    serde_json::from_str::<Vec<Container>>(&r#"
      [
        {
          "Id": "ed3221f4adc05b9ecfbf56b1aa76d4e6e70d5b73b3876c322fc10d017c64ca86",
          "Names": ["/rust"],
          "Image": "ghmlee/rust:latest",
          "Command": "bash",
          "Created": 1439434052,
          "Ports": [
            {
              "IP": "0.0.0.0",
              "PrivatePort": 8888,
              "PublicPort": 8888,
              "Type": "tcp"
            }
          ],
          "SizeRootFs": 253602755,
          "Labels": {},
          "Status": "Exited (137) 12 hours ago",
          "HostConfig": {
            "NetworkMode": "default"
          },
          "SizeRw": 10832473
        }
      ]
    "#).unwrap();
  }
}
