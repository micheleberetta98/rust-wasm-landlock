use crate::args::{get_args, Args};
use crate::landlock::Landlock;
use crate::path_access::PathAccess;
use crate::wasm::WasmModule;
use anyhow::Result;
use std::time::Instant;

mod args;
mod landlock;
mod path_access;
mod wasm;

fn main() -> Result<()> {
  let now = Instant::now();
  let args = get_args();
  let after_args = now.elapsed();

  // Read before enforcing landlock, otherwise we have to specify read permissions
  // for the executable folder too
  let mut module = WasmModule::new(&args.wasm_module)?;
  let after_instance = now.elapsed();

  module = module
    .use_stdio()
    .preopen_all(&args.dirs)?
    .preopen_all_map(&args.mapdirs)?;
  let after_preopen = now.elapsed();

  // Enforce landlock
  if !args.no_landlock {
    Landlock::new()?
      .add_rules(get_all_allows(&args))?
      .enforce()?;
  }
  let after_landlock = now.elapsed();

  let result = module.run();
  let after_run = now.elapsed();

  println!("Times:");
  println!("  {:?}", after_args);
  println!("  {:?}", after_instance - after_args);
  println!("  {:?}", after_preopen - after_instance);
  println!("  {:?}", after_landlock - after_preopen);
  println!("  {:?}", after_run - after_landlock);

  result
}

fn get_all_allows(args: &Args) -> impl Iterator<Item = PathAccess> + '_ {
  args
    .fs_allows
    .iter()
    .map(|(p, a)| PathAccess::new(p).allow(a))
}
