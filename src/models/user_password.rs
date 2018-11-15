#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserPassword {
  username: String,
  password: String,
  email: String,
  serveraddress: String,
}

impl UserPassword {
  pub fn new<S: Into<String>>(username: S, password: S, email: S, serveraddress: S) -> Self {
    Self {
      username: username.into(),
      password: password.into(),
      email: email.into(),
      serveraddress: serveraddress.into(),
    }
  }
}
