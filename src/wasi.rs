use anyhow::Result;
use std::fs;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::Dir;

pub fn run(module_path: &str, dir: &str) -> Result<()> {
  let engine = Engine::default();
  let mut linker = Linker::new(&engine);
  wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

  // Create a WASI context and put it in a Store; all instances in the store
  // share this context. `WasiCtxBuilder` provides a number of ways to
  // configure what the target program will have access to.
  let fd = fs::File::open(dir)?;
  let wasi = WasiCtxBuilder::new()
    .inherit_stdio()
    .inherit_args()?
    .preopened_dir(Dir::from_std_file(fd), ".")?
    .build();
  let mut store = Store::new(&engine, wasi);

  // Try and read some files
  let module = Module::from_file(&engine, module_path)?;
  linker.module(&mut store, "", &module)?;
  linker
    .get_default(&mut store, "")?
    .typed::<(), (), _>(&store)?
    .call(&mut store, ())?;

  Ok(())
}
