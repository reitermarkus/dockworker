use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use hyper::header::{ContentType};
use hyper::mime::{Mime, SubLevel, TopLevel};
use serde_json;
use tar::Archive;
use url;

use docker::{Docker, HaveHttpClient};
use error::{Error, Result};
use http_client::HttpClient;
use models::{Image, ImageId, ImageStatus, PrunedImages, RemovedImage};
use super::{DockerAPIError, api_result, ignore_result};

impl Docker {
  /// Create an image by pulling it from registry
  ///
  /// `/images/create`
  ///
  /// # NOTE
  /// When control returns from this function, creating job may not have been completed.
  /// For waiting the completion of the job, cunsuming response like `image_create("hello-world", "linux").map(|r| r.for_each(|_| ()));`.
  ///
  /// # TODO
  /// - Typing result iterator like image::ImageStatus.
  /// - Generalize input parameters
  pub fn image_create(
    &self,
    image: &str,
    tag: &str,
  ) -> Result<Box<Iterator<Item = Result<ImageStatus>>>> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair("fromImage", image);
    param.append_pair("tag", tag);

    let mut headers = self.headers().clone();
    self.credential().map(|credential| headers.set(credential.clone()));
    let res =
      self.http_client()
          .post(&headers, format!("/images/create?{}", param.finish()), "")?;
    if res.status.is_success() {
      Ok(Box::new(BufReader::new(res).lines().map(|line| {
        Ok(line?).and_then(|ref line| {
          println!("{}", line);
          Ok(serde_json::from_str(line)?)
        })
      })))
    } else {
      Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into())
    }
  }

  /// Push an image
  ///
  /// `/images/{name}/push`
  ///
  /// # NOTE
  /// For pushing an image to non default registry, add registry id to prefix of the image name like `<registry>/<image>` .
  /// But the name of the local cache image is `<image>:<tag>` .
  pub fn image_push(&self, name: &str, tag: &str) -> Result<()> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair("tag", tag);
    let mut headers = self.headers().clone();
    self.credential().map(|credential| headers.set(credential.clone()));
    self.http_client()
        .post(&headers, format!("/images/{}/push?{}", name, param.finish()), "")
        .and_then(ignore_result)
  }

  /// Remove an image
  ///
  /// `/images/{name}`
  pub fn image_delete(
    &self,
    name: &str,
    force: Option<bool>,
    noprune: Option<bool>,
  ) -> Result<Vec<RemovedImage>> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair("force", &force.unwrap_or(false).to_string());
    param.append_pair("noprune", &noprune.unwrap_or(false).to_string());
    self.http_client()
        .delete(
          self.headers(),
          format!("/images/{}?{}", name, param.finish()),
        )
        .and_then(api_result)
  }

  /// Delete unused images
  ///
  /// `/images/prune`
  pub fn image_prune(&self, dangling: bool) -> Result<PrunedImages> {
    let mut param = url::form_urlencoded::Serializer::new(String::new());
    param.append_pair(
      "filters",
      &format!(r#"{{"filters": {{"dangling":{}}} }}"#, dangling.to_string()),
    );
    self.http_client()
      .post(
        self.headers(),
        format!("/images/prune?{}", param.finish()),
        "",
      )
      .and_then(api_result)
  }

  /// List images
  ///
  /// `/images/json`
  pub fn image_list(&self, all: bool) -> Result<Vec<Image>> {
    self.http_client()
        .get(self.headers(), format!("/images/json?a={}", all as u32))
        .and_then(api_result)
  }

  /// Get a tarball containing all images and metadata for a repository
  ///
  /// `/images/{name}/get`
  pub fn image_export(&self, name: &str) -> Result<Box<Read>> {
    self.http_client()
        .get(self.headers(), format!("/images/{}/get", name))
        .and_then(|res| {
          if res.status.is_success() {
            Ok(Box::new(res) as Box<Read>)
          } else {
            Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into())
          }
        })
  }

  /// Import images
  ///
  /// `/images/load`
  ///
  /// # Summary
  /// Load a set of images and tags into a repository
  pub fn image_load(&self, quiet: bool, path: &Path) -> Result<ImageId> {
    let mut headers = self.headers().clone();
    let application_tar = Mime(TopLevel::Application, SubLevel::Ext("x-tar".into()), vec![]);
    headers.set(ContentType(application_tar));
    let res =
      self.http_client()
          .post_file(&headers, format!("/images/load?quiet={}", quiet), path)?;
    if !res.status.is_success() {
      return Err(serde_json::from_reader::<_, DockerAPIError>(res)?.into());
    }
    // read and discard to end of response
    for line in BufReader::new(res).lines() {
      let buf = line?;
      debug!("{}", buf);
    }

    let mut ar = Archive::new(File::open(path)?);
    for entry in ar.entries()?.filter_map(|e| e.ok()) {
      let path = entry.path()?;
      // looking for file name like XXXXXXXXXXXXXX.json
      if path.extension() == Some(OsStr::new("json")) && path != Path::new("manifest.json") {
        let stem = path.file_stem().unwrap(); // contains .json
        let id = stem.to_str()
                     .ok_or(Error::Unknown(format!("convert to String: {:?}", stem)))?;
        return Ok(ImageId::new(id.to_string()));
      }
    }
    Err(Error::Unknown("no expected file: XXXXXX.json".to_owned()).into())
  }
}
