use std::collections::HashMap as Map;
use std::path::PathBuf;
use std::time::Duration;

use models::{ContainerHostConfig, NetworkingConfig};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreateOptions {
  hostname: String,
  domainname: String,
  user: String,
  attach_stdin: bool,
  attach_stdout: bool,
  attach_stderr: bool,
  // exposed_ports: Map<String, Any>, not sure the type that this would need to be
  tty: bool,
  open_stdin: bool,
  stdin_once: bool,
  env: Vec<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  cmd: Vec<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  entrypoint: Vec<String>,
  image: String,
  labels: Map<String, String>,
  // volumes: Map<String, Any>, not sure the type that this would need to be.
  // healthcheck: Not sure the type that this would be
  working_dir: PathBuf,
  network_disabled: bool,
  mac_address: String,
  on_build: Vec<String>,
  stop_signal: String,
  #[serde(with = "format::duration::DurationDelegate")]
  stop_timeout: Duration,
  host_config: Option<ContainerHostConfig>,
  networking_config: Option<NetworkingConfig>,
}

mod format {
  pub mod duration {
    use std::time::Duration;

    #[derive(Serialize, Deserialize)]
    #[serde(remote = "Duration")]
    pub struct DurationDelegate(#[serde(getter = "Duration::as_secs")] u64);

    // Provide a conversion to construct the remote type.
    impl From<DurationDelegate> for Duration {
      fn from(def: DurationDelegate) -> Duration {
        Duration::new(def.0, 0)
      }
    }
  }
}

impl ContainerCreateOptions {
  pub fn new(image: &str) -> Self {
    Self {
      hostname: "".to_owned(),
      domainname: "".to_owned(),
      user: "".to_owned(),
      attach_stdin: false,
      attach_stdout: true,
      attach_stderr: true,
      tty: false,
      open_stdin: false,
      stdin_once: false,
      env: vec![],
      cmd: vec![],
      image: image.to_owned(),
      working_dir: PathBuf::new(),
      entrypoint: vec![],
      network_disabled: false,
      mac_address: "".to_owned(),
      on_build: vec![],
      labels: Map::new(),
      stop_signal: "SIGTERM".to_owned(),
      stop_timeout: Duration::from_secs(10),
      host_config: None,
      networking_config: None,
    }
  }

  pub fn hostname(&mut self, hostname: String) -> &mut Self {
    self.hostname = hostname;
    self
  }

  pub fn domainname(&mut self, domainname: String) -> &mut Self {
    self.domainname = domainname;
    self
  }

  pub fn user(&mut self, user: String) -> &mut Self {
    self.user = user;
    self
  }

  pub fn attach_stdin(&mut self, attach_stdin: bool) -> &mut Self {
    self.attach_stdin = attach_stdin;
    self
  }

  pub fn attach_stdout(&mut self, attach_stdout: bool) -> &mut Self {
    self.attach_stdout = attach_stdout;
    self
  }

  pub fn attach_stderr(&mut self, attach_stderr: bool) -> &mut Self {
    self.attach_stderr = attach_stderr;
    self
  }

  pub fn tty(&mut self, tty: bool) -> &mut Self {
    self.tty = tty;
    self
  }

  pub fn open_stdin(&mut self, open_stdin: bool) -> &mut Self {
    self.open_stdin = open_stdin;
    self
  }

  pub fn stdin_once(&mut self, stdin_once: bool) -> &mut Self {
    self.stdin_once = stdin_once;
    self
  }

  /// push back an envvar entry
  pub fn env(&mut self, env: String) -> &mut Self {
    self.env.push(env);
    self
  }

  /// push back a cmd argment
  pub fn cmd(&mut self, cmd: String) -> &mut Self {
    self.cmd.push(cmd);
    self
  }

  /// update entrypoint
  pub fn entrypoint(&mut self, entrypoint: Vec<String>) -> &mut Self {
    self.entrypoint = entrypoint;
    self
  }

  pub fn image(&mut self, image: String) -> &mut Self {
    self.image = image;
    self
  }

  /// add a label/value pair
  pub fn label(&mut self, key: String, value: String) -> &mut Self {
    self.labels.insert(key, value);
    self
  }

  pub fn working_dir(&mut self, working_dir: PathBuf) -> &mut Self {
    self.working_dir = working_dir;
    self
  }

  pub fn network_disabled(&mut self, network_disabled: bool) -> &mut Self {
    self.network_disabled = network_disabled;
    self
  }

  pub fn mac_address(&mut self, mac_address: String) -> &mut Self {
    self.mac_address = mac_address;
    self
  }

  pub fn on_build(&mut self, on_build: Vec<String>) -> &mut Self {
    self.on_build = on_build;
    self
  }

  pub fn stop_signal(&mut self, stop_signal: String) -> &mut Self {
    self.stop_signal = stop_signal;
    self
  }

  pub fn stop_timeout(&mut self, stop_timeout: Duration) -> &mut Self {
    self.stop_timeout = stop_timeout;
    self
  }

  pub fn host_config(&mut self, host_config: ContainerHostConfig) -> &mut Self {
    self.host_config = Some(host_config);
    self
  }

  pub fn networking_config(&mut self, networking_config: NetworkingConfig) -> &mut Self {
    self.networking_config = Some(networking_config);
    self
  }
}
