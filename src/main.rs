use crate::args::get_args;
use crate::path_access::PathAccess;
use anyhow::Result;

mod args;
mod landlock;
mod path_access;
mod wasi;

fn main() -> Result<()> {
  let args = get_args();
  let mut ruleset = landlock::create_ruleset()?;

  println!("WASM module to run: {}", args.wasm_module);
  println!("Preopened dir: {}", args.dir);

  for (path, access) in args.fs_allows {
    let pa = PathAccess::new(&path, access);
    ruleset = ruleset.add_rules(pa.iter())?;
  }

  let status = ruleset.restrict_self()?;
  landlock::guard_is_supported(status)?;

  wasi::run(&args.wasm_module, &args.dir)
}
