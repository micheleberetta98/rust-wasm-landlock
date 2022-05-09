use anyhow::Result;
use landlock::{AccessFs, BitFlags, PathBeneath, PathFd, PathFdError, RulesetError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathEnvError {
  #[error(transparent)]
  Ruleset(#[from] RulesetError),
  #[error(transparent)]
  AddRuleIter(#[from] PathFdError),
}

// A struct able to iterate landlock's PathBeneath rules
#[derive(Debug)]
pub struct PathAccess {
  paths: String,
  access: BitFlags<AccessFs>,
}

impl PathAccess {
  pub fn new(paths: &str, access: BitFlags<AccessFs>) -> Self {
    Self {
      paths: paths.to_owned(),
      access: access,
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = Result<PathBeneath<PathFd>, PathEnvError>> + '_ {
    self
      .paths
      .split(':')
      .skip_while(|s| s.is_empty())
      .map(move |path| to_path_beneath(path, self.access))
  }
}

fn to_path_beneath(
  path: &str,
  access: BitFlags<AccessFs>,
) -> Result<PathBeneath<PathFd>, PathEnvError> {
  let path_fd = PathFd::new(path)?;
  let path_beneath = PathBeneath::new(path_fd).allow_access(access);
  Ok(path_beneath)
}
