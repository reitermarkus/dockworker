#[derive(Debug, PartialEq, PartialOrd, Serialize)]
pub struct ContainerFilters {
  #[serde(skip_serializing_if = "Vec::is_empty")]
  id: Vec<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  name: Vec<String>,
}

impl Default for ContainerFilters {
  fn default() -> Self {
    Self {
      id: vec![],
      name: vec![],
    }
  }
}

impl ContainerFilters {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn id(&mut self, id: &str) -> &mut Self {
    self.id.push(id.to_owned());
    self
  }

  pub fn name(&mut self, name: &str) -> &mut Self {
    self.name.push(name.to_owned());
    self
  }
}
