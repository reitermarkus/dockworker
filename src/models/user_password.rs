use header::XRegistryAuth;

#[derive(Debug, Clone, Serialize)]
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

impl From<UserPassword> for XRegistryAuth {
  fn from(user_password: UserPassword) -> Self {
    Self::new(serde_json::to_string(&user_password).unwrap_or_default())
  }
}
