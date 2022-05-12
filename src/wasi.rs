use anyhow::Result;
use std::fs;
use std::fs::File;
use wasmtime::*;
use wasmtime_wasi::{Dir, WasiCtxBuilder};

pub struct WasmModule {
  bytes: Vec<u8>,
  ctx_builder: WasiCtxBuilder,
}

impl WasmModule {
  pub fn new(path: &str) -> Result<Self> {
    Ok(Self {
      bytes: fs::read(path)?,
      ctx_builder: WasiCtxBuilder::new(),
    })
  }

  pub fn use_stdio(mut self) -> Self {
    self.ctx_builder = self.ctx_builder.inherit_stdio();
    self
  }

  pub fn preopen_all(self, dirs: &Vec<String>) -> Result<Self> {
    let mapdirs: Vec<_> = dirs
      .iter()
      .map(|d| (d.to_string(), d.to_string()))
      .collect();

    self.preopen_all_map(&mapdirs)
  }

  pub fn preopen_all_map(mut self, mapdirs: &Vec<(String, String)>) -> Result<Self> {
    for (dir, guest) in mapdirs {
      self = self.preopen(&dir, &guest)?;
    }
    Ok(self)
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
    let module = Module::new(&engine, &self.bytes)?;
    linker.module(&mut store, "", &module)?;
    linker
      .get_default(&mut store, "")?
      .typed::<(), (), _>(&store)?
      .call(&mut store, ())?;

    Ok(())
  }

  pub fn invoke<Params, Results>(self, func_name: &str, params: Params) -> Result<Results>
  where
    Params: WasmParams,
    Results: WasmResults,
  {
    let wasi_ctx = self.ctx_builder.build();
    let engine = Engine::default();

    let mut store = Store::new(&engine, wasi_ctx);
    let module = Module::new(&engine, &self.bytes)?;

    // Here we create the instance with no imports (maybe add wasi to be more like above?)
    let instance = Instance::new(&mut store, &module, &[])?;

    let func = instance
      .get_func(&mut store, func_name)
      .expect(&format!("`{}` was not exported", func_name))
      .typed::<Params, Results, _>(&store)?;

    let result = func.call(&mut store, params)?;
    Ok(result)
  }
}
