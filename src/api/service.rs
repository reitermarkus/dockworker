use url;

use docker::{Docker, HaveHttpClient};
use error::Result;
use http_client::HttpClient;
use models::{Service, ServiceSpec};
use super::{api_result, ignore_result};

impl Docker {
  /// List services
  ///
  /// `/services`
  pub fn service_list(&self, id: Option<&str>, label: Option<&str>, mode: Option<&str>, name: Option<&str>) -> Result<Vec<Service>> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());

    let filters = json!({
      "id": id,
      "label": label,
      "mode": mode,
      "name": name,
    }).to_string();

    param.append_pair("filters", &filters);

    self.http_client()
        .get(&self.headers(), &format!("/services?{}", param.finish()))
        .and_then(api_result)
  }

  /// Inspect a service
  ///
  /// `/services/{id}`
  pub fn service_inspect(&self, id: &str, insert_defaults: Option<bool>) -> Result<Service> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());

    insert_defaults.map(|insert_defaults| param.append_pair("insertDefaults", &insert_defaults.to_string()));

    self.http_client()
        .get(&self.headers(), &format!("/services/{}?{}", id, param.finish()))
        .and_then(api_result)
  }

  /// Update a service
  ///
  /// `/services/{id}/update`
  pub fn service_update(&self,
    id: &str,
    version: u64, registry_auth_from: Option<&str>, rollback: Option<&str>,
    spec: &ServiceSpec,
  ) -> Result<()> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());

    param.append_pair("version", &version.to_string());

    if let Some(registry_auth_from) = registry_auth_from {
      param.append_pair("registryAuthFrom", &registry_auth_from);
    }

    if let Some(rollback) = rollback {
      param.append_pair("rollback", &rollback);
    }

    let mut headers = self.headers().clone();
    if let Some(credential) = self.credential() {
      headers.set(credential.clone());
    }

    let body = serde_json::to_string(&spec)?;

    self.http_client()
        .post(&headers, &format!("/services/{}/update?{}", id, param.finish()), &body)
        .and_then(ignore_result)
  }
}
