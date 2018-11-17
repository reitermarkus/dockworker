use std::io::Read;
use std::path::Path;
use std::time::Duration;

use hyper::header::ContentType;
use tar::Archive;
use url;

use container::AttachResponse;
use docker::{Docker, HaveHttpClient};
use error::Result;
use http_client::HttpClient;
use models::{Container, ContainerInfo, CreateContainerResponse, ContainerCreateOptions, ContainerFilters, ExitStatus, FilesystemChange, Top};
use signal::Signal;
use stats::StatsStream;
use super::{DockerAPIError, api_result, ignore_result, no_content};

impl Docker {
  /// Remove a container
  ///
  /// `/containers/{id}`
  pub fn container_delete(&self,
    id: &str,
    volume: Option<bool>,
    force: Option<bool>,
    link: Option<bool>,
  ) -> Result<()> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());

    volume.map(|volume| param.append_pair("v", &volume.to_string()));
    force.map(|force| param.append_pair("force", &force.to_string()));
    link.map(|link| param.append_pair("link", &link.to_string()));

    self.http_client()
        .delete(self.headers(), format!("/containers/{}?{}", id, param.finish()))
        .and_then(no_content)
  }

  /// Get an archive of a filesystem resource in a container
  ///
  /// `/containers/{id}/archive`
  pub fn container_archive(&self, id: &str, path: &Path) -> Result<Archive<Box<Read>>> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair("path", path.to_str().unwrap_or("")); // FIXME: cause an invalid path error
    self.http_client()
        .get(self.headers(), format!("/containers/{}/archive?{}", id, param.finish()))
        .and_then(|res| {
          if res.status.is_success() {
            Ok(Archive::new(Box::new(res) as Box<Read>))
          } else {
            Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into())
          }
        })
  }

  /// Extract an archive of files or folders to a directory in a container
  ///
  /// `/containers/{id}/archive`
  ///
  /// # Summary
  /// Extract given src file into the container specified with id.
  /// The input file must be a tar archive with id(no compress), gzip, bzip2 or xz.
  ///
  /// * id  : container name or ID
  /// * src : path to a source *file*
  /// * dst : path to a *directory* in the container to extract the archive's contents into
  pub fn put_container_archive(
    &self,
    id: &str,
    src: &Path,
    dst: &Path,
    no_overwrite_dir_non_dir: bool,
  ) -> Result<()> {
    debug!(
      "put_file({}, {}, {}, {})",
      id,
      src.display(),
      dst.display(),
      no_overwrite_dir_non_dir
    );
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair("path", &dst.to_string_lossy());
    param.append_pair("noOverwriteDirNonDir", &no_overwrite_dir_non_dir.to_string());
    self.http_client()
        .put_file(
          self.headers(),
          format!("/containers/{}/archive?{}", id, param.finish()),
          src,
        )
        .and_then(ignore_result)
  }

  /// List containers
  ///
  /// `/containers/json`
  pub fn container_list(
    &self,
    all: Option<bool>,
    limit: Option<u64>,
    size: Option<bool>,
    filters: ContainerFilters,
  ) -> Result<Vec<Container>> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair("all", &(all.unwrap_or(false) as u64).to_string());
    if let Some(limit) = limit {
      param.append_pair("limit", &limit.to_string());
    }
    param.append_pair("size", &(size.unwrap_or(false) as u64).to_string());
    param.append_pair("filters", &serde_json::to_string(&filters).unwrap());
    debug!("filter: {}", serde_json::to_string(&filters).unwrap());

    self.http_client()
        .get(self.headers(), format!("/containers/json?{}", param.finish()))
        .and_then(api_result)
  }

  /// Inspect about a container
  ///
  /// `/containers/{id}/json`
  pub fn container_inspect(&self, container: &Container) -> Result<ContainerInfo> {
    self.http_client()
        .get(self.headers(), format!("/containers/{}/json", container.id))
        .and_then(api_result)
  }

  /// Get changes on a container's filesystem
  ///
  /// `/containers/{id}/changes`
  pub fn container_changes(&self, container: &Container) -> Result<Vec<FilesystemChange>> {
    self.http_client()
        .get(self.headers(), format!("/containers/{}/changes", container.id))
        .and_then(api_result)
  }

  /// Export a container
  ///
  /// `/containers/{id}/export`
  ///
  /// # Summary
  /// Returns a pointer to tar archive stream.
  pub fn container_export(&self, container: &Container) -> Result<Box<Read>> {
    self.http_client()
        .get(self.headers(), format!("/containers/{}/export", container.id))
        .and_then(|res| {
          if res.status.is_success() {
            Ok(Box::new(res) as Box<Read>)
          } else {
            Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into())
          }
        })
  }

  /// Create a container
  ///
  /// `/containers/create`
  ///
  /// # Summary
  ///
  /// * `name` - None: auto naming
  /// * `option` - create options
  pub fn container_create(
    &self,
    name: Option<&str>,
    option: &ContainerCreateOptions,
  ) -> Result<CreateContainerResponse> {
    let path = match name {
      Some(name) => {
        let mut param = url::form_urlencoded::Serializer::new(String::new());
        param.append_pair("name", name);
        format!("/containers/create?{}", param.finish())
      }
      None => format!("/containers/create"),
    };

    let json_body = serde_json::to_string(&option)?;
    let mut headers = self.headers().clone();
    headers.set(ContentType::json());
    self.http_client()
        .post(&headers, &path, &json_body)
        .and_then(api_result)
  }


  /// Start a container
  ///
  /// `/containers/{id}/start`
  pub fn container_start(&self, id: &str) -> Result<()> {
    self.http_client()
        .post(self.headers(), format!("/containers/{}/start", id), "")
        .and_then(no_content)
  }

  /// Stop a container
  ///
  /// `/containers/{id}/stop`
  pub fn container_stop(&self, id: &str, timeout: Duration) -> Result<()> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair("t", &timeout.as_secs().to_string());
    self.http_client()
        .post(
          self.headers(),
          format!("/containers/{}/stop?{}", id, param.finish()),
          "",
        )
        .and_then(no_content)
  }

  /// Kill a container
  ///
  /// `/containers/{id}/kill`
  pub fn container_kill(&self, id: &str, signal: Signal) -> Result<()> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair("signal", &signal.as_i32().to_string());
    self.http_client()
        .post(self.headers(), format!("/containers/{}/kill?{}", id, param.finish()), "")
        .and_then(no_content)
  }

  /// Attach to a container
  ///
  /// `/containers/{id}/attach`
  ///
  /// Attach to a container to read its output or send it input.
  pub fn container_attach(
    &self,
    id: &str,
    detach_keys: Option<&str>,
    logs: bool,
    stream: bool,
    stdin: bool,
    stdout: bool,
    stderr: bool,
  ) -> Result<AttachResponse> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());

    detach_keys.map(|detach_keys| param.append_pair("detachKeys", detach_keys));
    param.append_pair("logs", &logs.to_string());
    param.append_pair("stream", &stream.to_string());
    param.append_pair("stdin", &stdin.to_string());
    param.append_pair("stdout", &stdout.to_string());
    param.append_pair("stderr", &stderr.to_string());

    self.http_client()
        .post(self.headers(), format!("/containers/{}/attach?{}", id, param.finish()), "")
        .and_then(|res| {
          if res.status.is_success() {
            Ok(AttachResponse::new(res))
          } else {
            Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into())
          }
        })
  }

  /// List processes running inside a container
  ///
  /// `/containers/{id}/top`
  pub fn container_top(&self, container: &Container, ps_args: Option<&str>) -> Result<Top> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());

    ps_args.map(|ps_args| param.append_pair("ps_args", ps_args));

    self.http_client()
        .get(self.headers(), format!("/containers/{}/top?{}", container.id, param.finish()))
        .and_then(api_result)
  }

  /// Get containers stats based resource usage
  ///
  /// `/containers/{id}/stats`
  pub fn container_stats(&self, container: &Container) -> Result<StatsStream> {
    let res = self.http_client()
                  .get(self.headers(), format!("/containers/{}/stats", container.id))?;
    Ok(StatsStream::new(res))
  }

  /// Wait for a container
  ///
  /// `/containers/{id}/wait`
  pub fn container_wait(&self, id: &str) -> Result<ExitStatus> {
      self.http_client()
          .post(self.headers(), format!("/containers/{}/wait", id), "")
          .and_then(api_result)
  }
}
