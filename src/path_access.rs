// A struct able to iterate landlock's PathBeneath rules
pub struct PathAccess {
  paths: String,
  access: Vec<u8>,
}

impl PathAccess {
  fn new(paths: &str, access: Vec<u8>) -> Self {
    Self {
      paths: paths.to_owned(),
      access: access,
    }
  }

  fn iter(&self) -> impl Iterator<Item = &str> {
    self.paths.split(':').skip_while(|s| s.is_empty())
  }
}
