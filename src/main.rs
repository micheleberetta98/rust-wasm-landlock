use anyhow::{Result};

mod wasi;
mod args;
mod path_access;

use crate::args::get_args;
use crate::path_access::PathAccess;

fn main() -> Result<()> {
  let args = get_args();
  println!("Preopened dir: {}", args.dir);
  for (path, access) in args.fs_allows {
    let landlock_path_access = PathAccess::new(&path, access);
    println!("{:?}", landlock_path_access)
  }
  Ok(())
}

/* fn run_wasi(dir: &str, path: &str) -> Result<()> {
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
    let module = Module::from_file(&engine, path)?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), (), _>(&store)?
        .call(&mut store, ())?;

    Ok(())
} */
