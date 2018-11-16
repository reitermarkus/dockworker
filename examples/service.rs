extern crate failure;
use failure::Error;

extern crate dockworker;
use dockworker::{Docker, models::{Mode, UserPassword}};

fn main() -> Result<(), Error> {
  let mut docker = Docker::from_env()?;

  let serveraddress = "localhost:5000";
  docker.set_credential(UserPassword::new(
      "someusername",
      "somepassword",
      "someusername@example.com",
      serveraddress,
  ));

  let services = docker.list_services(None, None, None, None)?;
  println!("{:#?}", services);



  for service in services {
    let id = service.id;
    let version = service.version.index;
    let mut spec = service.spec;

    println!("{} (v{})", id, version.to_string());
    println!("{:#?}", spec);

    spec.mode = Mode::Replicated { replicas: 2 };

    docker.update_service(&id, version, None, None, spec)?;
  }

  Ok(())
}
