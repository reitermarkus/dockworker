extern crate dockworker;
extern crate failure;

use failure::*;
use dockworker::{Docker, models::ContainerFilters};

fn find_all_exported_ports() -> Result<(), Error> {
  let docker = Docker::from_env()?;
  let filter = ContainerFilters::new();
  let containers = docker.container_list(None, None, None, filter)?;
  for container in &containers {
    let info = docker.container_inspect(&container)?;

    // Uncomment this to dump everything we know about a container.
    // println!("{:#?}", &info);

    if let Some(ports) = info.network_settings.ports {
      let ports: Vec<String> = ports.keys().cloned().collect();
      println!("{}: {}", &info.name, ports.join(", "));
    }
  }
  Ok(())
}

fn main() {
  if let Err(err) = find_all_exported_ports() {
    eprint!("Error: ");
    for e in err.iter_chain() {
      eprintln!("{}\n", e);
    }
  }
}
