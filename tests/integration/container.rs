use dockworker::*;
use std::fs::File;
use std::io::Read;
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::env;

use support::{file, with_image};

use tar::Builder as TarBuilder;

use dockworker::{Error, models::{AttachContainer, ContainerCreateOptions, ContainerHostConfig, ExitStatus}};

#[test]
fn create_delete() {
  let docker = Docker::from_env().unwrap();
  let (name, tag) = ("hello-world", "linux");
  assert!(
    docker.image_create(name, tag)
          .map(|sts| sts.for_each(|st| println!("{:?}", st)))
          .is_ok()
  );
  let mut create = ContainerCreateOptions::new(&format!("{}:{}", name, tag));
  create.host_config(ContainerHostConfig::new());

  assert!(
    docker.container_create(Some("dockworker_test"), &create)
          .is_ok()
  );
  assert!(
    docker.container_delete("dockworker_test", None, None, None)
          .is_ok()
  );
  assert!(
    docker.image_delete(&format!("{}:{}", name, tag), None, None)
          .is_ok()
  );
}

#[test]
fn auto_remove() {
  let docker = Docker::from_env().unwrap();
  let (name, tag) = ("alpine", "3.7");
  assert!(
    docker.image_create(name, tag)
          .map(|sts| sts.for_each(|st| println!("{:?}", st)))
          .is_ok()
  );
  let mut host_config = ContainerHostConfig::new();
  host_config.auto_remove(true);
  let mut create = ContainerCreateOptions::new(&format!("{}:{}", name, tag));
  create.host_config(host_config);

  let container = docker
    .container_create(Some("dockworker_auto_remove_container"), &create)
    .unwrap();
  assert!(docker.container_start(&container.id).is_ok());
  assert!(docker.container_wait(&container.id).is_ok());
  assert!(
    docker.container_delete("dockworker_auto_remove_container", None, None, None)
          .is_err() // 'no such container' or 'removel container in progress'
  );
  assert!(
    docker.image_delete(&format!("{}:{}", name, tag), Some(true), None)
          .is_ok()
  );
}

#[test]
fn wait() {
  let docker = Docker::from_env().unwrap();
  let (name, tag) = ("alpine", "3.4");
  let container_name = "alpine34_exit0";
  with_image(&docker, name, tag, |name, tag| {
    let mut create = ContainerCreateOptions::new(&format!("{}:{}", name, tag));
    create.cmd("ls".to_string());
    assert!(
      docker.container_create(Some(container_name), &create)
            .is_ok()
    );
    assert_eq!(
      docker.container_wait(container_name).unwrap(),
      ExitStatus::new(0)
    );
    assert!(
      docker.container_delete(container_name, None, None, None)
            .is_ok()
    );
  })
}

#[test]
fn put_archive() {
  let docker = Docker::from_env().unwrap();
  let (name, tag) = ("alpine", "3.6");

  let temp_dir = env::temp_dir();
  let test_file = &temp_dir.join("test_file");

  with_image(&docker, name, tag, |name, tag| {
    let create = ContainerCreateOptions::new(&format!("{}:{}", name, tag));
    let container = docker.container_create(None, &create).unwrap();
    assert!(docker.container_start(&container.id).is_ok());

    file::generate(test_file, 1024).unwrap();
    {
      let mut builder =
        TarBuilder::new(File::create(test_file.with_extension("tar")).unwrap());
      builder.append_file(
               test_file.strip_prefix("/").unwrap(),
               &mut File::open(test_file).unwrap(),
             )
             .unwrap();
    }

    assert!(match docker.container_archive(&container.id, test_file) {
      Ok(_) => false,
      Err(Error::API { .. }) => true, // not found
      Err(_) => false,
    });

    docker.put_container_archive(
            &container.id,
            &test_file.with_extension("tar"),
            Path::new("/"),
            true,
          )
          .unwrap();

    docker.container_archive(&container.id, test_file)
          .unwrap()
          .unpack(temp_dir.join("put"))
          .unwrap();

    docker.container_wait(&container.id).unwrap();

    docker.container_delete(&container.id, None, None, None)
          .unwrap();
  });

  assert!(file::equal(
    test_file,
    &temp_dir.join("put").join(test_file.file_name().unwrap())
  ));
}

/// This is executed after `docker-compose build iostream`
#[test]
#[ignore]
fn attach() {
  let docker = Docker::from_env().unwrap();

  // expected files
  let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docker");
  let exps: &[&str; 2] = &["./sample/apache-2.0.txt", "./sample/bsd4.txt"];
  let image_name = "test-iostream:latest";

  let host_config = ContainerHostConfig::new();
  //host_config.auto_remove(true);
  let mut create = ContainerCreateOptions::new(image_name);
  create.cmd(exps[0].to_owned())
        .cmd(exps[1].to_owned())
        .host_config(host_config);

  let container = docker.container_create(None, &create).unwrap();
  docker.container_start(&container.id).unwrap();
  let res = docker
    .container_attach(&container.id, None, true, true, false, true, true)
    .unwrap();
  let cont: AttachContainer = res.into();

  // expected files
  let exp_stdout = File::open(root.join(exps[0])).unwrap();
  let exp_stderr = File::open(root.join(exps[1])).unwrap();

  assert!(
    exp_stdout
      .bytes()
      .map(|e| e.ok())
      .eq(cont.stdout.bytes().map(|e| e.ok()))
  );
  assert!(
    exp_stderr
      .bytes()
      .map(|e| e.ok())
      .eq(cont.stderr.bytes().map(|e| e.ok()))
  );

  docker.container_wait(&container.id).unwrap();
  docker.container_delete(&container.id, None, None, None)
        .unwrap();
  docker.image_delete(image_name, None, None).unwrap();
}
