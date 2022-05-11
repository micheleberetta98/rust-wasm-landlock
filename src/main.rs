use crate::args::get_args;
use crate::path_access::PathAccess;
use crate::wasi::WasmModule;
use anyhow::Result;

mod args;
mod landlock;
mod path_access;
mod wasi;

fn main() -> Result<()> {
  let args = get_args();
  let mut ruleset = landlock::create_ruleset()?;

  println!("WASM module to run: {}", args.wasm_module);
  println!("Preopened dirs: {:?}", args.dirs);
  println!("Mapped dirs:    {:?}", args.mapdirs);

  for (path, access) in args.fs_allows {
    let pa = PathAccess::new(&path, access);
    ruleset = ruleset.add_rules(pa.iter())?;
  }

  let status = ruleset.restrict_self()?;
  landlock::guard_is_supported(status)?;

  WasmModule::new(&args.wasm_module)
    .use_stdio()
    .preopen_all(args.dirs)?
    .preopen_all_map(args.mapdirs)?
    .run()?;

  Ok(())
}
