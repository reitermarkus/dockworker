use std::collections::HashMap as Map;

use models::UnspecifiedObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
  pub attach_stderr: bool,
  pub attach_stdin: bool,
  pub attach_stdout: bool,
  // TODO: Verify that this is never just a `String`.
  //pub Cmd: Vec<String>,
  pub domainname: String,
  // TODO: The source says `Option<String>` but I've seen
  // `Option<Vec<String>>` on the wire.  Ignore until we figure it out.
  //pub Entrypoint: Option<Vec<String>>,
  pub env: Option<Vec<String>>,
  pub exposed_ports: Option<Map<String, UnspecifiedObject>>,
  pub hostname: String,
  pub image: String,
  pub labels: Map<String, String>,
  // TODO: We don't know exacly what this vec contains.
  //pub OnBuild: Option<Vec<???>>,
  pub open_stdin: bool,
  pub stdin_once: bool,
  pub tty: bool,
  pub user: String,
  pub volumes: Option<Map<String, UnspecifiedObject>>,
  pub working_dir: String,
}
