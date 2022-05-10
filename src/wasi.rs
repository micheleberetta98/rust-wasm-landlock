use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Dir, WasiCtx, WasiCtxBuilder};

pub struct WasmModule {
  path: String,
}

impl WasmModule {
  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_owned(),
    }
  }

  pub fn run(&self, wasi_ctx: WasiCtx) -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
    let mut store = Store::new(&engine, wasi_ctx);

    let module = Module::from_file(&engine, &self.path)?;
    linker.module(&mut store, "", &module)?;
    linker
      .get_default(&mut store, "")?
      .typed::<(), (), _>(&store)?
      .call(&mut store, ())?;
    Ok(())
  }
}
