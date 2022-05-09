use anyhow::{bail, Result};
use landlock::{
  Access, AccessFs, RestrictionStatus, Ruleset, RulesetCreated, RulesetError, RulesetStatus, ABI,
};

pub fn create_ruleset() -> Result<RulesetCreated, RulesetError> {
  Ruleset::new()
    .handle_access(AccessFs::from_all(ABI::V1))?
    .create()
}

pub fn guard_is_supported(status: RestrictionStatus) -> Result<()> {
  if status.ruleset == RulesetStatus::NotEnforced {
    bail!("Landlock not supported by the running kernel!");
  }
  Ok(())
}
