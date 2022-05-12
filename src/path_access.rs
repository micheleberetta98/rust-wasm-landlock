use anyhow::Result;
use landlock::{AccessFs, BitFlags, PathBeneath, PathFd, PathFdError, RulesetError};
use std::path::{Path, PathBuf};
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
  path: PathBuf,
  access: BitFlags<AccessFs>,
}

impl PathAccess {
  pub fn new(path: &str) -> Self {
    Self {
      path: Path::new(path).to_owned(),
      access: BitFlags::empty(),
    }
  }

  pub fn allow(mut self, access: &BitFlags<AccessFs>) -> Self {
    self.access |= *access;
    self
  }

  pub fn iter(&self) -> impl Iterator<Item = Result<PathBeneath<PathFd>, PathEnvError>> {
    std::iter::once(self.to_path_beneath())
  }

  fn to_path_beneath(&self) -> Result<PathBeneath<PathFd>, PathEnvError> {
    let path_fd = PathFd::new(&self.path)?;
    let path_beneath = PathBeneath::new(path_fd).allow_access(self.access);
    Ok(path_beneath)
  }
}
