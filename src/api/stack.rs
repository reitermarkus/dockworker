use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Lines};
use std::process::ChildStdout;

use docker::Docker;
use error::Result;

impl Docker {
  /// Deploy a stack
  ///
  /// `docker stack deploy`
  pub fn stack_deploy(&self, stack: &str, compose_file: Option<impl AsRef<Path>>, bundle_file: Option<impl AsRef<Path>>, prune: Option<bool>) -> Result<Lines<BufReader<ChildStdout>>> {
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
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::null());

    let child = cmd.spawn()?;

    Ok(BufReader::new(child.stdout.unwrap()).lines())
  }

  pub fn stack_deploy_with_bundle_file(&self, stack: &str, bundle_file: impl AsRef<Path>, prune: Option<bool>) -> Result<Lines<BufReader<ChildStdout>>> {
    self.stack_deploy(stack, None::<&Path>, Some(bundle_file), prune)
  }

  pub fn stack_deploy_with_compose_file(&self, stack: &str, compose_file: impl AsRef<Path>, prune: Option<bool>) -> Result<Lines<BufReader<ChildStdout>>> {
    self.stack_deploy(stack, Some(compose_file), None::<&Path>, prune)
  }
}
