use dockworker::*;

#[test]
#[cfg(feature = "docker_running")]
fn test_system_ping() {
  let docker = Docker::from_env().unwrap();
  assert!(docker.system_ping().is_ok());
}

#[test]
#[cfg(feature = "docker_running")]
fn test_system_info() {
  let docker = Docker::from_env().unwrap();
  assert!(docker.system_info().is_ok());
}

#[test]
#[cfg(feature = "docker_running")]
fn test_system_version() {
  let docker = Docker::from_env().unwrap();
  assert!(docker.system_version().is_ok());
}
