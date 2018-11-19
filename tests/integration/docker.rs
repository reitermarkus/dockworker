use dockworker::Docker;

#[test]
#[ignore]
fn build() {
  let docker = Docker::from_env().unwrap();
  let swarm = docker.swarm();
  swarm.init().finish().unwrap();
}
