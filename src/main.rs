use crate::args::{get_args, Args};
use crate::landlock::Landlock;
use crate::path_access::PathAccess;
use crate::wasm::WasmModule;
use anyhow::Result;

mod args;
mod landlock;
mod path_access;
mod wasm;

fn main() -> Result<()> {
  let args = get_args();

  if cfg!(debug_assertions) {
    println!("WASM module to run: {}", args.wasm_module);
    println!("Preopened dirs:     {:?}", args.dirs);
    println!("Mapped dirs:        {:?}", args.mapdirs);
    println!("Enable landlock:    {:?}", !args.no_landlock);
  }

  // Read before enforcing landlock, otherwise we have to specify read permissions
  // for the executable folder too
  let module = WasmModule::new(&args.wasm_module)?
    .use_stdio()
    .preopen_all(&args.dirs)?
    .preopen_all_map(&args.mapdirs)?;

  // Enforce landlock
  if !args.no_landlock {
    Landlock::new()?
      .add_rules(get_all_allows(&args))?
      .enforce()?;
  }

  module.run()
}

fn get_all_allows(args: &Args) -> impl Iterator<Item = PathAccess> + '_ {
  args
    .fs_allows
    .iter()
    .map(|(p, a)| PathAccess::new(p).allow(a))
}
