//! Docker
#![doc(html_root_url = "https://ghmlee.github.io/rust-docker/doc")]

extern crate base64;
extern crate byteorder;
extern crate dirs;
#[macro_use]
extern crate failure;
extern crate hyper;
#[macro_use]
extern crate log;
#[cfg(windows)]
extern crate named_pipe;
extern crate nix;
#[cfg(feature = "openssl")]
extern crate openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tar;
#[cfg(unix)]
extern crate unix_socket;
extern crate url;

mod header;
pub mod container;
mod docker;
pub mod error;
mod hyper_client;
pub mod process;
pub mod signal;
pub mod stats;
pub mod models;
mod test;
#[cfg(unix)]
mod unix;
mod util;

pub use docker::Docker;

mod serde_helpers;
