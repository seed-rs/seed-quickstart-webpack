pub struct Model {
  pub description: String,
  pub completed: bool,
}

impl Default for Model {
  fn default() -> Self {
    Self {
      description: "".to_owned(),
      completed: false,
    }
  }
}
