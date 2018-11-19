use hyper::client::response::Response;

/// Response of attach to container api
#[derive(Debug)]
pub struct AttachResponse {
  pub(crate) res: Response,
}

impl AttachResponse {
  pub fn new(res: Response) -> Self {
    Self { res }
  }
}
