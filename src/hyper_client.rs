use std::fs::File;
use std::path::Path;
use std::result;
#[cfg(feature = "openssl")]
use std::sync::Arc;

use hyper::client::pool::{Config, Pool};
use hyper::client::response::Response;
use hyper::client::IntoUrl;
use hyper::header::Headers;
use hyper::net::HttpConnector;
#[cfg(feature = "openssl")]
use hyper::net::{HttpsConnector, Openssl};
use hyper::Client;
use hyper::Url;
#[cfg(feature = "openssl")]
use openssl::{ssl::{SslContext, SslMethod}, x509::X509FileType};
use http_client::HttpClient;
use error::*;
#[cfg(unix)]
use unix::HttpUnixConnector;

/// Http client using hyper
#[derive(Debug)]
pub struct HyperClient {
    /// http client
    client: Client,
    /// base connection address
    base: Url,
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

impl HyperClient {
    fn new(client: Client, base: Url) -> Self {
        Self { client, base }
    }

    /// path to unix socket
    #[cfg(unix)]
    pub fn with_unix_socket<S: AsRef<str>>(path: S) -> Self {
        let conn = HttpUnixConnector::new(path.as_ref());
        let pool_config = Config { max_idle: 8 };
        let pool = Pool::with_connector(pool_config, conn);

        // dummy base address
        let base_addr = "http://localhost".into_url().expect("dummy base url");
        let client = Client::with_connector(pool);
        Self::new(client, base_addr)
    }

    #[cfg(feature = "openssl")]
    pub fn with_ssl(
        addr: &str,
        key: &Path,
        cert: &Path,
        ca: &Path,
    ) -> result::Result<Self, Error> {
        // This ensures that using docker-machine-esque addresses work with Hyper.
        let url = Url::parse(&addr.clone().replacen("tcp://", "https://", 1))?;

        let ctx = ssl_context(addr, key, cert, ca)?;
        let conn = HttpsConnector::new(ctx);
        let pool_config = Config { max_idle: 8 };
        let pool = Pool::with_connector(pool_config, conn);

        let client = Client::with_connector(pool);
        Ok(Self::new(client, url))
    }

    pub fn with_tcp(addr: &str) -> result::Result<Self, Error> {
        // This ensures that using docker-machine-esque addresses work with Hyper.
        let url = Url::parse(&addr.clone().replace("tcp://", "http://"))?;

        let conn = HttpConnector::default();
        let pool_config = Config { max_idle: 8 };
        let pool = Pool::with_connector(pool_config, conn);

        let client = Client::with_connector(pool);
        Ok(Self::new(client, url))
    }
}

impl HttpClient for HyperClient {
    type Err = Error;

    fn get<S: AsRef<str>>(&self, headers: &Headers, path: S) -> result::Result<Response, Self::Err> {
        let url = self.base.join(path.as_ref())?;
        let res = self.client.get(url).headers(headers.clone()).send()?;
        Ok(res)
    }

    fn post<S: AsRef<str>>(
        &self,
        headers: &Headers,
        path: S,
        body: &str,
    ) -> result::Result<Response, Self::Err> {
        let url = self.base.join(path.as_ref())?;
        let res = self.client
            .post(url)
            .headers(headers.clone())
            .body(body)
            .send()?;
        Ok(res)
    }

    fn delete<S: AsRef<str>>(&self, headers: &Headers, path: S) -> result::Result<Response, Self::Err> {
        let url = self.base.join(path.as_ref())?;
        let res = self.client.delete(url).headers(headers.clone()).send()?;
        Ok(res)
    }

    fn post_file<S: AsRef<str>>(
        &self,
        headers: &Headers,
        path: S,
        file: &Path,
    ) -> result::Result<Response, Self::Err> {
        let mut content = File::open(file)?;
        let url = self.base.join(path.as_ref())?;
        let res = self.client
            .post(url)
            .headers(headers.clone())
            .body(&mut content)
            .send()?;
        Ok(res)
    }

    fn put_file<S: AsRef<str>>(
        &self,
        headers: &Headers,
        path: S,
        file: &Path,
    ) -> result::Result<Response, Self::Err> {
        let mut content = File::open(file)?;
        let url = self.base.join(path.as_ref())?;
        let res = self.client
            .put(url)
            .headers(headers.clone())
            .body(&mut content)
            .send()?;
        Ok(res)
    }
}
