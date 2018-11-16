use std::fs::File;
use std::path::Path;
use std::result;
#[cfg(feature = "openssl")]
use std::sync::Arc;

use hyper::{Response, Request};
use hyper::HeaderMap;
use hyper::rt::Future;
//#[cfg(feature = "openssl")]
//use hyper::net::{HttpsConnector, Openssl};
use hyper::{Client, Body, Uri};
use futures::future::IntoFuture;
use hyper::client::connect::{Connect};
use http::uri::PathAndQuery;
#[cfg(feature = "openssl")]
use openssl::{ssl::{SslContext, SslMethod}, x509::X509FileType};
use http_client::HttpClient;
use error::*;

#[derive(Debug)]
pub struct HyperClient<C> {
  client: Client<C, Body>,
  base: Uri,
}

#[cfg(feature = "openssl")]
fn ssl_context(addr: &str, key: &Path, cert: &Path, ca: &Path) -> result::Result<Openssl, Error> {
  let mkerr = |_| Error::SslError(addr.to_owned());
  let mut context = SslContext::new(SslMethod::Sslv23).map_err(mkerr)?;
  context.set_CA_file(ca).map_err(mkerr)?;
  context
    .set_certificate_file(cert, X509FileType::PEM)
    .map_err(mkerr)?;
  context
    .set_private_key_file(key, X509FileType::PEM)
    .map_err(mkerr)?;
  Ok(Openssl {
    context: Arc::new(context),
  })
}

use hyperlocal;

impl<C> HyperClient<C>
where
  C: Connect + Sync
{
  fn new(client: Client<C, Body>, base: Uri) -> Self {
    Self { client, base }
  }


  /// path to unix socket
  #[cfg(unix)]
  pub fn with_unix_socket<S: Clone + AsRef<str> + AsRef<Path>>(path: S) -> Self {
    let uri: Uri = hyperlocal::Uri::new(path.clone(), "/").into();

    Self::new(
      Client::builder()
        .build::<_, hyper::Body>(hyperlocal::UnixConnector::new()),
      uri,
    )
  }

  // #[cfg(feature = "openssl")]
  // pub fn with_ssl(
  //   addr: &str,
  //   key: &Path,
  //   cert: &Path,
  //   ca: &Path,
  // ) -> result::Result<Self, Error> {
  //   // This ensures that using docker-machine-esque addresses work with Hyper.
  //   let url = Url::parse(&addr.clone().replacen("tcp://", "https://", 1))?;
  //   let conn = ssl_context(addr, key, cert, ca).map(HttpsConnector::new)?;
  //   let client = Client::with_connector(conn);
  //   Ok(Self::new(client, url))
  // }
  //
  // pub fn with_tcp(addr: &str) -> result::Result<Self, Error> {
  //   // This ensures that using docker-machine-esque addresses work with Hyper.
  //   let url = Url::parse(&addr.clone().replace("tcp://", "http://"))?;
  //   let conn = HttpConnector::default();
  //   let client = Client::with_connector(conn);
  //   Ok(Self::new(client, url))
  // }
}

impl<C> HttpClient for HyperClient<C>
where
  C: Connect + Sync
{
  type Err = Error;

  fn get<S: AsRef<str>>(&self, headers: &HeaderMap, path: S) -> result::Result<Response<Body>, Self::Err> {
    let mut parts = self.base.into_parts();
    parts.path_and_query = path.as_ref().parse().ok();
    let uri = Uri::from_parts(parts)?;

    let mut request = Request::builder()
      .uri(uri)
      .method("GET")
      .body(Body::empty())
      .unwrap();

    for (name, value) in headers {
      request.headers_mut().insert(name, value.clone());
    }

    self.client.request(request).wait().map_err(|e| e.into())
  }

  fn post<S: AsRef<str>>(
    &self,
    headers: &HeaderMap,
    path: S,
    body: &str,
  ) -> result::Result<Response<Body>, Self::Err> {
    let mut parts = self.base.into_parts();
    parts.path_and_query = path.as_ref().parse().ok();
    let uri = Uri::from_parts(parts)?;

    let mut request = Request::post(uri)
      .body(String::from(body).into())
      .unwrap();

    for (name, value) in headers {
      request.headers_mut().insert(name, value.clone());
    }

    self.client.request(request).wait().map_err(|e| e.into())
  }

  fn delete<S: AsRef<str>>(&self, headers: &HeaderMap, path: S) -> result::Result<Response<Body>, Self::Err> {
    let mut parts = self.base.into_parts();
    parts.path_and_query = path.as_ref().parse().ok();
    let uri = Uri::from_parts(parts)?;

    let mut request = Request::delete(uri)
      .body(Body::empty())
      .unwrap();

    for (name, value) in headers {
      request.headers_mut().insert(name, value.clone());
    }

    self.client.request(request).wait().map_err(|e| e.into())
  }

  fn post_file<S: AsRef<str>>(
      &self,
      headers: &HeaderMap,
      path: S,
      file: &Path,
  ) -> result::Result<Response<Body>, Self::Err> {
    let mut parts = self.base.into_parts();
    parts.path_and_query = path.as_ref().parse().ok();
    let uri = Uri::from_parts(parts)?;

    let mut content = File::open(file)?;

    let mut request = Request::post(uri)
      .body(Body::wrap_stream(&mut content))
      .unwrap();

    for (name, value) in headers {
      request.headers_mut().insert(*name, *value);
    }

    self.client.request(request).wait().map_err(|e| e.into())
  }

  fn put_file<S: AsRef<str>>(
    &self,
    headers: &HeaderMap,
    path: S,
    file: &Path,
  ) -> result::Result<Response<Body>, Self::Err> {
    let mut parts = self.base.into_parts();
    parts.path_and_query = path.as_ref().parse().ok();
    let uri = Uri::from_parts(parts)?;

    let mut content = File::open(file)?;

    let mut request = Request::put(uri)
      .body(Body::wrap_stream(&mut content))
      .unwrap();

    for (name, value) in headers {
      request.headers_mut().insert(*name, *value);
    }

    self.client.request(request).wait().map_err(|e| e.into())
  }
}
