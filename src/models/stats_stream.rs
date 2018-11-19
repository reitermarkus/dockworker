use hyper::client::response::Response;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

use serde_json;

use models::Stats;
use error::*;

#[derive(Debug)]
pub struct StatsStream {
  buf: BufReader<Response>,
}

impl StatsStream {
  pub fn new(response: Response) -> Self {
    Self {
      buf: BufReader::new(response),
    }
  }
}

impl Iterator for StatsStream {
  type Item = Result<Stats>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut line = String::new();
    match self.buf.read_line(&mut line) {
      Ok(0) => None,
      Ok(_) => Some(serde_json::from_str(&line).map_err(|e| e.into())),
      Err(err) => Some(Err(err.into())),
    }
  }
}
