use crate::args::get_args;
use crate::path_access::PathAccess;
use crate::wasi::WasmModule;
use anyhow::Result;
use wasmtime_wasi::{Dir, WasiCtxBuilder};

mod args;
mod landlock;
mod path_access;
mod wasi;

fn main() -> Result<()> {
  let args = get_args();
  let mut ruleset = landlock::create_ruleset()?;

  println!("WASM module to run: {}", args.wasm_module);
  println!("Preopened dir: {:?}", args.dir);

  for (path, access) in args.fs_allows {
    let pa = PathAccess::new(&path, access);
    ruleset = ruleset.add_rules(pa.iter())?;
  }

  let status = ruleset.restrict_self()?;
  landlock::guard_is_supported(status)?;

  let mut wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args()?;
  if let Some(dir) = args.dir {
    let fd = std::fs::File::open(dir)?;
    wasi = wasi.preopened_dir(Dir::from_std_file(fd), ".")?;
  }

  let ctx = wasi.build();
  WasmModule::new(&args.wasm_module).run(ctx)?;

  Ok(())
}
