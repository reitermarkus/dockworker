use std::path::Path;
use std::fs::{remove_file, File};
use std::io;

use dockworker::*;

use support::{pull_image};

#[test]
fn create_delete() {
  let docker = Docker::from_env().unwrap();
  let (name, tag) = ("debian", "latest");
  let sts = docker
    .image_create(name, tag)
    .map(|sts| sts.for_each(|st| println!("{:?}", st)));
  assert!(sts.is_ok());
  assert!(
    docker.image_delete(&format!("{}:{}", name, tag), None, None)
          .is_ok()
  );
}

#[test]
fn export_load() {
  let docker = Docker::from_env().unwrap();
  pull_image(&docker, "alpine", "latest");

  {
    let mut file = File::create("dockworker_test_alpine.tar").unwrap();
    let mut res = docker.image_export("alpine:latest").unwrap();
    io::copy(&mut res, &mut file).unwrap();
  }

  assert!(docker.image_delete("alpine:latest", None, None).is_ok());
  assert!(
    docker.image_load(false, Path::new("dockworker_test_alpine.tar"))
          .is_ok()
  );
  assert!(remove_file("dockworker_test_alpine.tar").is_ok());
}
