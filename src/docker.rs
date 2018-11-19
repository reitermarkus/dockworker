use hyper::client::IntoUrl;
use hyper::header::Headers;
use std::env;
use std::path::{Path, PathBuf};

use models::*;
use http_client::HttpClient;
use error::*;
use hyper_client::HyperClient;
use process::Process;
use header::XRegistryAuth;

use swarm::Swarm;

/// The default `DOCKER_HOST` address that we will try to connect to.
#[cfg(unix)]
pub const DEFAULT_DOCKER_HOST: &'static str = "unix:///var/run/docker.sock";

/// The default `DOCKER_HOST` address that we will try to connect to.
///
/// This should technically be `"npipe:////./pipe/docker_engine"` on
/// Windows, but we don't support Windows pipes yet.  However, the TCP port
/// is still available.
#[cfg(windows)]
pub const DEFAULT_DOCKER_HOST: &'static str = "tcp://localhost:2375";

/// Handle to connection to the docker daemon
#[derive(Debug)]
pub struct Docker {
  pub(crate) client: HyperClient,
  headers: Headers,
  credential: Option<XRegistryAuth>,
}

/// Access to inner HttpClient
pub trait HaveHttpClient {
  type Client: HttpClient;
  fn http_client(&self) -> &Self::Client;
}

impl Docker {
    fn new(client: HyperClient) -> Self {
        Self {
            client,
            headers: Headers::new(),
            credential: None,
        }
    }

    pub fn set_credential<A: Into<XRegistryAuth>>(&mut self, auth: A) {
      self.credential = Some(auth.into())
    }

    pub(crate) fn headers(&self) -> &Headers {
        &self.headers
    }

    pub(crate) fn credential(&self) -> Option<XRegistryAuth> {
        self.credential.clone()
    }

    /// Connect to the Docker daemon using the standard Docker
    /// configuration options. This includes `DOCKER_HOST`,
    /// `DOCKER_TLS_VERIFY`, `DOCKER_CERT_PATH` and `DOCKER_CONFIG`, and we
    /// try to interpret these as much like the standard `docker` client as
    /// possible.
    pub fn from_env() -> Result<Docker> {
      let host = env::var("DOCKER_HOST").unwrap_or(DEFAULT_DOCKER_HOST.to_string());

      // Dispatch to the correct connection function.
      let err = Error::CouldNotConnect(host.clone());
      if host.starts_with("unix://") {
        return Docker::with_unix_socket(&host).map_err(|_| err)
      }

      if host.starts_with("tcp://") {
        let tls_verify = env::var("DOCKER_TLS_VERIFY").ok().filter(|s| !s.is_empty());
        let cert_path = env::var("DOCKER_CERT_PATH").ok().filter(|s| !s.is_empty());

        if tls_verify.is_some() || cert_path.is_some() {
          let cert_path = match cert_path {
            Some(path) => PathBuf::from(&path),
            None => dirs::home_dir().ok_or(Error::NoCertPath)?.join(".docker"),
          };

          return Docker::with_ssl(
            &host,
            &cert_path.join("key.pem"),
            &cert_path.join("cert.pem"),
            &cert_path.join("ca.pem"),
          ).map_err(|_| err)
        }

        return Docker::with_tcp(&host).map_err(|_| err)
      }

      Err(Error::UnsupportedScheme(host.clone()).into())
    }

    #[cfg(unix)]
    pub fn with_unix_socket(addr: &str) -> Result<Docker> {
        // This ensures that using a fully-qualified path --
        // e.g. unix://.... -- works.  The unix socket provider expects a
        // Path, so we don't need scheme.
        let url = addr.into_url()?;
        let client = HyperClient::with_unix_socket(url.path());
        Ok(Docker::new(client))
    }

    #[cfg(not(unix))]
    pub fn with_unix_socket(addr: &str) -> Result<Docker> {
        Err(Error::UnsupportedScheme(addr.to_owned()))
    }

    #[cfg(feature = "openssl")]
    pub fn with_ssl(addr: &str, key: &Path, cert: &Path, ca: &Path) -> Result<Docker> {
        let client = HyperClient::with_ssl(addr, key, cert, ca)?;
        Ok(Docker::new(client))
    }

    #[cfg(not(feature = "openssl"))]
    pub fn with_ssl(_addr: &str, _key: &Path, _cert: &Path, _ca: &Path) -> Result<Docker> {
        Err(Error::SslDisabled)
    }

    /// Connect using unsecured HTTP.  This is strongly discouraged
    /// everywhere but on Windows when npipe support is not available.
    pub fn with_tcp(addr: &str) -> Result<Docker> {
        let client = HyperClient::with_tcp(addr)?;
        Ok(Docker::new(client))
    }

    pub fn processes(&self, container: &Container) -> Result<Vec<Process>> {
        let top = self.container_top(container, None)?;

        Ok(top.processes
            .iter()
            .map(|process| {
                let mut p = Process::default();

                for (i, value) in process.iter().enumerate() {
                    let v = value.clone();

                    match top.titles[i].as_ref() {
                        "UID" => p.user = v,
                        "USER" => p.user = v,
                        "PID" => p.pid = v,
                        "%CPU" => p.cpu = Some(v),
                        "%MEM" => p.memory = Some(v),
                        "VSZ" => p.vsz = Some(v),
                        "RSS" => p.rss = Some(v),
                        "TTY" => p.tty = Some(v),
                        "STAT" => p.stat = Some(v),
                        "START" => p.start = Some(v),
                        "STIME" => p.start = Some(v),
                        "TIME" => p.time = Some(v),
                        "CMD" => p.command = v,
                        "COMMAND" => p.command = v,
                        _ => {}
                    }
                }
                p
            })
            .collect())
    }

  pub fn swarm(&self) -> Swarm {
    Swarm::new(&self)
  }
}

impl HaveHttpClient for Docker {
  type Client = HyperClient;

  fn http_client(&self) -> &Self::Client {
    &self.client
  }
}
