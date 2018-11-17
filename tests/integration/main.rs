#![deny(warnings)]

extern crate dockworker;

extern crate hyper;
extern crate rand;
extern crate serde_json;
extern crate tar;

#[macro_use]
mod support;

mod container;
mod image;
mod stats;
