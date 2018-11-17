use serde_aux::prelude::*;

use models::{Component, SystemVersionResponsePlatform};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemVersionResponse {
  pub platform: SystemVersionResponsePlatform,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub components: Vec<Component>,
  pub version: String,
  pub api_version: String,
  #[serde(rename = "MinAPIVersion")]
  pub min_api_version: String,
  pub git_commit: String,
  pub go_version: String,
  pub os: String,
  pub arch: String,
  pub kernel_version: String,
  pub experimental: Option<bool>,
  pub build_time: Option<String>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    serde_json::from_str::<SystemVersionResponse>(&r#"
      {
        "Platform": {
          "Name": ""
        },
        "Components": [
          {
            "Name": "Engine",
            "Version": "18.06.1-ce",
            "Details": {
              "ApiVersion": "1.38",
              "Arch": "amd64",
              "BuildTime": "2018-08-21T17:29:02.000000000+00:00",
              "Experimental": "true",
              "GitCommit": "e68fc7a",
              "GoVersion": "go1.10.3",
              "KernelVersion": "4.9.93-linuxkit-aufs",
              "MinAPIVersion": "1.12",
              "Os": "linux"
            }
          }
        ],
        "Version": "18.06.1-ce",
        "ApiVersion": "1.38",
        "MinAPIVersion": "1.12",
        "GitCommit": "e68fc7a",
        "GoVersion": "go1.10.3",
        "Os": "linux",
        "Arch": "amd64",
        "KernelVersion": "4.9.93-linuxkit-aufs",
        "Experimental": true,
        "BuildTime": "2018-08-21T17:29:02.000000000+00:00"
      }
    "#).unwrap();
  }
}
