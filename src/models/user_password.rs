#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserPassword {
  username: String,
  password: String,
  email: String,
  serveraddress: String,
}

impl UserPassword {
  pub fn new(username: String, password: String, email: String, serveraddress: String) -> Self {
    Self {
      username,
      password,
      email,
      serveraddress,
    }
  }
}
