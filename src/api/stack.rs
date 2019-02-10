use std::path::Path;
use std::process::{Command, Stdio};

use docker::Docker;
use error::{Error, Result};

impl Docker {
  /// Deploy a stack
  ///
  /// `docker stack deploy`
  pub fn stack_deploy(&self, stack: &str, compose_file: Option<impl AsRef<Path>>, bundle_file: Option<impl AsRef<Path>>, prune: Option<bool>) -> Result<()> {
    let mut cmd = Command::new("docker");

    cmd.args(&["stack", "deploy"]);

    if let Some(bundle_file) = bundle_file {
      cmd.arg("--bundle-file");
      cmd.arg(bundle_file.as_ref());
    }

    if let Some(compose_file) = compose_file {
      cmd.arg("--compose-file");
      cmd.arg(compose_file.as_ref());
    }

    if prune == Some(true) {
      cmd.arg("--prune");
    }

    cmd.arg(stack);

    cmd.stdin(Stdio::null());

    let mut child = cmd.spawn()?;

    let exit_status = child.wait()?;

    if exit_status.success() {
      Ok(())
    } else {
      Err(Error::Unknown("docker stack deploy failed".into()))
    }
  }

  pub fn stack_deploy_with_bundle_file(&self, stack: &str, bundle_file: impl AsRef<Path>, prune: Option<bool>) -> Result<()> {
    self.stack_deploy(stack, None::<&Path>, Some(bundle_file), prune)
  }

  pub fn stack_deploy_with_compose_file(&self, stack: &str, compose_file: impl AsRef<Path>, prune: Option<bool>) -> Result<()> {
    self.stack_deploy(stack, Some(compose_file), None::<&Path>, prune)
  }
}
