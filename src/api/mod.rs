use std::io::Read;

use hyper::client::response::Response;
use hyper::status::StatusCode;
use serde::de::DeserializeOwned;
use serde_json;

use error::{Error, Result};

pub mod container;
pub mod image;
pub mod secret;
pub mod service;
pub mod stack;
pub mod swarm;
pub mod system;

#[derive(Debug, Deserialize)]
pub(crate) struct DockerAPIError {
  pub message: String,
}

impl From<DockerAPIError> for Error {
  fn from(e: DockerAPIError) -> Self {
    Error::API { message: e.message }
  }
}

/// Deserialize from json string
pub(crate) fn api_result<D: DeserializeOwned>(res: Response) -> Result<D> {
  if res.status.is_success() {
    Ok(serde_json::from_reader::<_, D>(res)?)
  } else {
    Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into())
  }
}

/// Expect 204 NoContent
pub(crate) fn no_content(res: Response) -> Result<()> {
  if res.status == StatusCode::NoContent {
    Ok(())
  } else {
    Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into())
  }
}

/// Ignore succeed response
///
/// Read whole response body, then ignore it.
pub(crate) fn ignore_result(res: Response) -> Result<()> {
  if res.status.is_success() {
    res.bytes().last(); // ignore
    Ok(())
  } else {
    Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into())
  }
}

pub(crate) fn string_result(mut res: Response) -> Result<String> {
  if res.status.is_success() {
    let mut buf = String::new();
    res.read_to_string(&mut buf)?;
    Ok(buf)
  } else {
    Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into())
  }
}
