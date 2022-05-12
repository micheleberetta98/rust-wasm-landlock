use crate::args::{get_args, Args};
use crate::landlock::Landlock;
use crate::path_access::PathAccess;
use crate::wasi::WasmModule;
use anyhow::Result;

mod args;
mod landlock;
mod path_access;
mod wasi;

fn main() -> Result<()> {
  let args = get_args();

  println!("WASM module to run: {}", args.wasm_module);
  println!("Preopened dirs: {:?}", args.dirs);
  println!("Mapped dirs:    {:?}", args.mapdirs);

  Landlock::new()?
    .add_rules(get_all_allows(&args))?
    .enforce()?;

  WasmModule::new(&args.wasm_module)
    .use_stdio()
    .preopen_all(args.dirs)?
    .preopen_all_map(args.mapdirs)?
    .run()?;

  Ok(())
}

fn get_all_allows(args: &Args) -> Vec<PathAccess> {
  args
    .fs_allows
    .iter()
    .map(|(p, a)| PathAccess::new(p).allow(a))
    .collect()
}
