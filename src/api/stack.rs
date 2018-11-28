use std::path::Path;
use std::process::Command;

use docker::Docker;
use error::Result;

impl Docker {
  /// Deploy a stack
  ///
  /// `docker stack deploy`
  pub fn stack_deploy(&self, stack: &str, compose_file: Option<&Path>, bundle_file: Option<&Path>, prune: Option<bool>) -> Result<String> {
    let mut cmd = Command::new("docker");

    cmd.args(&["stack", "deploy"]);

    if let Some(bundle_file) = bundle_file {
      cmd.arg("--bundle-file");
      cmd.arg(bundle_file);
    }

    if let Some(compose_file) = compose_file {
      cmd.arg("--compose-file");
      cmd.arg(compose_file);
    }

    if prune == Some(true) {
      cmd.arg("--prune");
    }

    cmd.arg(stack);

    Ok(String::from_utf8(cmd.output()?.stdout)?)
  }
}
