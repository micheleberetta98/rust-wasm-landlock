use anyhow::Result;

mod args;
mod path_access;
mod wasi;

use crate::args::get_args;
use crate::path_access::PathAccess;

fn main() -> Result<()> {
  let args = get_args();
  println!("WASM module to run: {}", args.wasm_module);
  println!("Preopened dir: {}", args.dir);
  for (path, access) in args.fs_allows {
    let landlock_path_access = PathAccess::new(&path, access);
    println!("{:?}", landlock_path_access)
  }

  wasi::run(&args.wasm_module, &args.dir)
}
