use hyper::header::ContentType;
use url;

use docker::{Docker, HaveHttpClient};
use error::Result;
use http_client::HttpClient;
use models::{AuthConfig, SystemAuthResponse, SystemDataUsageResponse, SystemEventsResponse, SystemInfo, SystemVersionResponse};
use super::{api_result, string_result};

impl Docker {
  /// Check auth configuration
  ///
  /// `/auth`
  pub fn system_auth(&self, auth_config: AuthConfig) -> Result<SystemAuthResponse> {
    let json_body = serde_json::to_string(&auth_config)?;

    let mut headers = self.headers().clone();
    headers.set(ContentType::json());

    self.http_client()
        .post(&headers, "/auth", &json_body)
        .and_then(api_result)
  }

  /// Get data usage information
  ///
  /// `/system/df`
  pub fn system_data_usage(&self) -> Result<SystemDataUsageResponse> {
    self.http_client()
        .get(self.headers(), "/system/df")
        .and_then(api_result)
  }

  /// Monitor events
  ///
  /// `/events`
  pub fn system_events(&self, since: Option<&str>, until: Option<&str>, filters: Option<&str>) -> Result<SystemEventsResponse> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());

    since.map(|since| param.append_pair("since", &since));
    until.map(|until| param.append_pair("until", &until));
    filters.map(|filters| param.append_pair("filters", &filters));

    self.http_client()
        .get(self.headers(), format!("/events?{}", param.finish()))
        .and_then(api_result)
  }

  /// Get system information
  ///
  /// `/info`
  pub fn system_info(&self) -> Result<SystemInfo> {
    self.http_client()
        .get(self.headers(), "/info")
        .and_then(api_result)
  }

  /// Test if the server is accessible
  ///
  /// `/_ping`
  pub fn system_ping(&self) -> Result<String> {
    self.http_client()
        .get(self.headers(), "/_ping")
        .and_then(string_result)
  }

  /// Get version and various information
  ///
  /// `/version`
  pub fn system_version(&self) -> Result<SystemVersionResponse> {
    self.http_client()
        .get(self.headers(), "/version")
        .and_then(api_result)
  }
}
