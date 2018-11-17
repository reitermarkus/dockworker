extern crate dockworker;
extern crate hyper;

use dockworker::{Docker, models::AuthConfig};

fn main() {
  let docker = Docker::from_env().unwrap();
  let token = docker.system_auth(AuthConfig::new(
                      "someusername",
                      "somepassword",
                      "someusername@example.com",
                      "localhost:5000",
                    ))
                    .unwrap();
  println!("token: {:?}", token);
}
