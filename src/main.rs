use anyhow::{bail, Result};
use landlock::{
    make_bitflags, Access, AccessFs, BitFlags, PathBeneath, PathFd, PathFdError, Ruleset,
    RulesetError, RulesetStatus, ABI,
};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use thiserror::Error;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::Dir;

use clap::Parser;

const ACCESS_FS_ROUGHLY_READ: BitFlags<AccessFs> = make_bitflags!(AccessFs::{ ReadFile | ReadDir });

#[derive(Debug, Error)]
enum PathEnvError {
    #[error(transparent)]
    Ruleset(#[from] RulesetError),
    #[error(transparent)]
    AddRuleIter(#[from] PathFdError),
}

struct PathEnv {
    paths: Vec<u8>,
    access: BitFlags<AccessFs>,
}

impl PathEnv {
    /// Create an object able to iterate PathBeneath rules
    ///
    /// # Arguments
    ///
    /// * `name`: String identifying an environment variable containing paths requested to be
    ///   allowed. Paths are separated with ":", e.g. "/bin:/lib:/usr:/proc". In case an empty
    ///   string is provided, NO restrictions are applied.
    /// * `access`: Set of access-rights allowed for each of the parsed paths.
    fn new<'a>(name: &str, access: BitFlags<AccessFs>) -> Result<Self, PathEnvError> {
        Ok(Self {
            paths: String::from(name).into_bytes(),
            access,
        })
    }

    fn iter(&self) -> impl Iterator<Item = Result<PathBeneath<PathFd>, PathEnvError>> + '_ {
        let is_empty = self.paths.is_empty();
        self.paths
            .split(|b| *b == b':')
            // Skips the first empty element from of an empty string.
            .skip_while(move |_| is_empty)
            .map(OsStr::from_bytes)
            .map(move |path| Ok(PathBeneath::new(PathFd::new(path)?).allow_access(self.access)))
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    name: String,
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
    // let status = Ruleset::new()
    //     .handle_access(AccessFs::from_all(ABI::V1))?
    //     .create()?
    //     .add_rules(PathEnv::new("tmp-dir", ACCESS_FS_ROUGHLY_READ)?.iter())?
    //     .restrict_self()
    //     .expect("Failed to enforce ruleset");

    // if status.ruleset == RulesetStatus::NotEnforced {
    //     bail!("Landlock is not supported by the running kernel.");
    // }

    // Reading works ok
    // let content =
    //   fs::read_to_string("tmp-dir/hello.txt").expect("Something went wrong reading file!");
    // println!("{}", content);

    // This does give permission denied!
    // touch(Path::new("hello.txt"))?;

    // Try and read some files
    // run_wasi("./tmp-dir", "./tmp-dir/read-file.wasm")?;

    // Try and create some files
    // run_wasi("./tmp-dir", "./tmp-dir/touch-file.wasm")?;

    Ok(())
}

fn run_wasi(dir: &str, path: &str) -> Result<()> {
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
}
