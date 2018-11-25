use std::env;
use std::path::{Path, PathBuf};

use hyper::header::Headers;
use url::Url;

use models::*;
use http_client::HttpClient;
use error::*;
use hyper_client::HyperClient;
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
    let host = env::var("DOCKER_HOST").ok().filter(|s| !s.is_empty())
                                      .and_then(|s| Url::parse(&s).ok())
                                      .unwrap_or(Url::parse(DEFAULT_DOCKER_HOST).unwrap());

    if let Some(docker) = Self::from_unix_socket_url(&host) {
      return Ok(docker)
    }

    // Dispatch to the correct connection function.
    let err = Error::CouldNotConnect(host.to_string());

    if host.scheme() == "tcp" {
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

      return Ok(Docker::with_tcp(&host))
    }

    Err(Error::UnsupportedScheme(host.to_string()).into())
  }


  #[cfg(unix)]
  fn from_unix_socket_url(url: &Url) -> Option<Docker> {
    if url.scheme() == "unix" {
      return Some(Docker::with_unix_socket(&url.path()))
    }

    None
  }

  #[cfg(not(unix))]
  fn from_unix_socket_url(url: &Url) -> Option<Docker> {
    None
  }

  #[cfg(unix)]
  pub fn with_unix_socket(addr: impl AsRef<Path>) -> Docker {
    Docker::new(HyperClient::with_unix_socket(addr))
  }

  #[cfg(feature = "openssl")]
  pub fn with_ssl(url: &Url, key: &Path, cert: &Path, ca: &Path) -> Result<Docker> {
    let client = HyperClient::with_ssl(url, key, cert, ca)?;
    Ok(Docker::new(client))
  }

  #[cfg(not(feature = "openssl"))]
  pub fn with_ssl(_url: &Url, _key: &Path, _cert: &Path, _ca: &Path) -> Result<Docker> {
    Err(Error::SslDisabled)
  }

  /// Connect using unsecured HTTP.  This is strongly discouraged
  /// everywhere but on Windows when npipe support is not available.
  pub fn with_tcp(addr: &Url) -> Docker {
    Docker::new(HyperClient::with_tcp(addr))
  }

  pub fn processes(&self, container: &Container) -> Result<Vec<Process>> {
    Ok(self.container_top(container, None)?.into())
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
