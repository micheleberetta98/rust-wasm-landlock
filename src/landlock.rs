use crate::path_access::PathAccess;
use anyhow::{bail, Result};
use landlock::{
  Access, AccessFs, RestrictionStatus, Ruleset, RulesetCreated, RulesetError, RulesetStatus, ABI,
};

pub struct Landlock {
  ruleset: RulesetCreated,
}

impl Landlock {
  pub fn new() -> Result<Self, RulesetError> {
    let ruleset = Ruleset::new()
      .handle_access(AccessFs::from_all(ABI::V1))?
      .create()?;

    Ok(Self { ruleset: ruleset })
  }

  pub fn add_rules(mut self, rules: impl Iterator<Item = PathAccess>) -> Result<Self> {
    for rule in rules {
      self = self.add_rule(rule)?;
    }
    Ok(self)
  }

  pub fn add_rule(mut self, path_access: PathAccess) -> Result<Self> {
    self.ruleset = self.ruleset.add_rules(path_access.iter())?;
    Ok(self)
  }

  pub fn enforce(self) -> Result<RestrictionStatus> {
    let status = self.ruleset.restrict_self()?;
    if status.ruleset == RulesetStatus::NotEnforced {
      bail!("Landlock not supported by the running kernel!");
    }
    Ok(status)
  }
}
