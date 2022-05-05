// A struct able to iterate landlock's PathBeneath rules
pub struct PathAccess {
  paths: String,
  access: u64,
}

impl PathAccess {
  pub fn new(paths: &str, access: u64) -> Self {
    Self {
      paths: paths.to_owned(),
      access: access,
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &str> {
    self.paths.split(':').skip_while(|s| s.is_empty())
  }
}
