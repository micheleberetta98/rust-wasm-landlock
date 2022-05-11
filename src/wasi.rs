use anyhow::Result;
use std::fs::File;
use wasmtime::*;
use wasmtime_wasi::{Dir, WasiCtxBuilder};

pub struct WasmModule {
  path: String,
  ctx_builder: WasiCtxBuilder,
}

impl WasmModule {
  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_owned(),
      ctx_builder: WasiCtxBuilder::new(),
    }
  }

  pub fn use_stdio(mut self) -> Self {
    self.ctx_builder = self.ctx_builder.inherit_stdio();
    self
  }

  pub fn preopen(mut self, dir: &str, guest_path: &str) -> Result<Self> {
    let fd = File::open(dir)?;
    self.ctx_builder = self
      .ctx_builder
      .preopened_dir(Dir::from_std_file(fd), guest_path)?;
    Ok(self)
  }

  pub fn run(self) -> Result<()> {
    let wasi_ctx = self.ctx_builder.build();
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
