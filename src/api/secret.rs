use hyper::header::ContentType;

use docker::{Docker, HaveHttpClient};
use error::Result;
use http_client::HttpClient;
use models::{IdResponse, Secret, SecretInspect};
use super::api_result;

impl Docker {
  /// Get secrets
  ///
  /// `/secrets`
  pub fn secret_list(&self) -> Result<Vec<Secret>> {
    self.http_client()
        .get(self.headers(), "/secrets")
        .and_then(api_result)
  }

  /// Get secrets
  ///
  /// `/secrets/create`
  pub fn secret_create(&self, name: &str, data: &str) -> Result<IdResponse> {
    let data = json!({
      "Name": name,
      "Data": base64::encode(data),
    }).to_string();

    let mut headers = self.headers().clone();
    headers.set(ContentType::json());

    self.http_client()
        .post(&self.headers(), &"/secrets/create", &data)
        .and_then(api_result)
  }

  /// Inspect a secret
  ///
  /// `/secrets/{id}`
  pub fn secret_inspect(&self, id: &str) -> Result<SecretInspect> {
    self.http_client()
        .get(&self.headers(), &format!("/secrets/{}", id))
        .and_then(api_result)
  }
}
