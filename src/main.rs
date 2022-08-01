use crate::args::{get_args, Args};
use crate::landlock::Landlock;
use crate::path_access::PathAccess;
use crate::wasm::WasmModule;
use anyhow::Result;
use std::fs;
use std::io::Write;
use std::time::Instant;

mod args;
mod landlock;
mod path_access;
mod wasm;

fn main() -> Result<()> {
  let filename = std::env::var("FILE").expect("$FILE must be set");
  let mut file = fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open(filename)
    .unwrap();

  let len = 5;
  let mut cumulative = vec![0; len];
  let mut delta = vec![0; len];

  let now = Instant::now();
  let args = get_args();
  cumulative[0] = now.elapsed().as_micros();

  // Read before enforcing landlock, otherwise we have to specify read permissions
  // for the executable folder too
  let mut module = WasmModule::new(&args.wasm_module)?;
  cumulative[1] = now.elapsed().as_micros();

  module = module
    .use_stdio()
    .preopen_all(&args.dirs)?
    .preopen_all_map(&args.mapdirs)?;
  cumulative[2] = now.elapsed().as_micros();

  // Enforce landlock
  if !args.no_landlock {
    Landlock::new()?
      .add_rules(get_all_allows(&args))?
      .enforce()?;
  }
  cumulative[3] = now.elapsed().as_micros();

  let result = module.run();
  cumulative[4] = now.elapsed().as_micros();

  delta[0] = cumulative[0];
  for i in 1..len {
    delta[i] = cumulative[i] - cumulative[i - 1];
  }

  let s = format!("{},{}\n", to_csv_line(delta), to_csv_line(cumulative));
  file.write_all(s.as_bytes())?;

  result
}

fn get_all_allows(args: &Args) -> impl Iterator<Item = PathAccess> + '_ {
  args
    .fs_allows
    .iter()
    .map(|(p, a)| PathAccess::new(p).allow(a))
}

fn to_csv_line<T: std::fmt::Display>(v: Vec<T>) -> String {
  v.iter()
    .map(|t| t.to_string())
    .collect::<Vec<_>>()
    .join(",")
}
