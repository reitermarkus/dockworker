extern crate dockworker;

use dockworker::errors::*;
use dockworker::{ContainerListOptions, Docker};
use std::io::{self, Write};

fn find_all_exported_ports() -> Result<()> {
    let docker = Docker::from_env()?;
    let containers = docker.containers(ContainerListOptions::default())?;
    for container in &containers {
        let info = docker.container_info(&container)?;

        // Uncomment this to dump everything we know about a container.
        //println!("{:#?}", &info);

        if let Some(ports) = info.network_settings.ports {
            let ports: Vec<String> = ports.keys().cloned().collect();
            println!("{}: {}", &info.name, ports.join(", "));
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = find_all_exported_ports() {
        write!(io::stderr(), "Error: ").unwrap();
        for e in err.iter() {
            write!(io::stderr(), "{}\n", e).unwrap();
        }
    }
}
