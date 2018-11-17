use hyper::header::ContentType;
use url;

use docker::{Docker, HaveHttpClient};
use error::Result;
use http_client::HttpClient;
use models::{Swarm, SwarmSpec, UnlockKeyResponse};
use super::{api_result, ignore_result};

impl Docker {
  /// Initialize a new swarm
  ///
  /// `/swarm/init`
  pub fn swarm_init(&self, advertise_addr: Option<String>, listen_addr: Option<String>, force_new_cluster: Option<bool>, spec: Option<SwarmSpec>) -> Result<String> {
    let data = json!({
      "AdvertiseAddr": advertise_addr,
      "ListenAddr": listen_addr.unwrap_or("0.0.0.0".to_string()),
      "ForceNewCluster": force_new_cluster,
      "Spec": spec,
    }).to_string();

    let mut headers = self.headers().clone();
    headers.set(ContentType::json());

    self.http_client()
        .post(&headers, "/swarm/init", &data)
        .and_then(api_result)
  }

  /// Inspect a swarm
  ///
  /// `/swarm`
  pub fn swarm_inspect(&self) -> Result<Swarm> {
    self.http_client()
        .get(self.headers(), "/swarm")
        .and_then(api_result)
  }

  pub fn swarm_join(&self, listen_addr: String, advertise_addr: String, data_path_addr: String, remote_addrs: Vec<String>, join_token: String) -> Result<()> {
    let data = json!({
      "ListenAddr": listen_addr,
      "AdvertiseAddr": advertise_addr,
      "DataPathAddr": data_path_addr,
      "RemoteAddrs": remote_addrs,
      "JoinToken": join_token,
    }).to_string();

    let mut headers = self.headers().clone();
    headers.set(ContentType::json());

    self.http_client()
        .post(&headers, "/swarm/join", &data)
        .and_then(api_result)
  }

  /// Leave swarm
  ///
  /// `/swarm/leave`
  pub fn swarm_leave(&self, force: bool) -> Result<()> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair("force", &force.to_string());

    self.http_client()
        .post(&self.headers(), format!("/swarm/leave?{}", param.finish()), "")
        .and_then(ignore_result)
  }

  /// Update a swarm
  ///
  /// `/swarm/update`
  pub fn swarm_update(&self, version: i64, rotate_worker_token: Option<bool>, rotate_manager_token: Option<bool>, rotate_manager_unlock_key: Option<bool>, spec: SwarmSpec) -> Result<()> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());

    param.append_pair("version", &version.to_string());
    rotate_worker_token.map(|rotate_worker_token|
      param.append_pair("rotateWorkerToken", &rotate_worker_token.to_string())
    );
    rotate_manager_token.map(|rotate_manager_token|
      param.append_pair("rotateManagerToken", &rotate_manager_token.to_string())
    );
    rotate_manager_unlock_key.map(|rotate_manager_unlock_key|
      param.append_pair("rotateManagerUnlockKey", &rotate_manager_unlock_key.to_string())
    );

    let mut headers = self.headers().clone();
    headers.set(ContentType::json());

    self.http_client()
        .post(&headers, format!("/swarm/update?{}", param.finish()), &serde_json::to_string(&spec).unwrap())
        .and_then(ignore_result)
  }

  /// Get the unlock key
  ///
  /// `/swarm/unlockkey`
  pub fn swarm_unlockkey(&self) -> Result<UnlockKeyResponse> {
    self.http_client()
        .get(self.headers(), "/unlockkey")
        .and_then(api_result)
  }

  /// Unlock a locked manager
  ///
  /// `/swarm/unlock`
  pub fn swarm_unlock(&self, unlock_key: String) -> Result<()> {
    let data = json!({
      "UnlockKey": unlock_key,
    }).to_string();

    let mut headers = self.headers().clone();
    headers.set(ContentType::json());

    self.http_client()
        .post(&headers, "/swarm/unlock", &data)
        .and_then(ignore_result)
  }
}
