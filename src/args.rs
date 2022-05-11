use anyhow::{bail, Result};
use clap::Parser;
use landlock::AccessFs::*;
use landlock::{make_bitflags, AccessFs, BitFlags};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  // The module to execute
  pub wasm_module: String,

  // The preopepend dir(s) to pass to wasmtime
  #[clap(long = "dir", short, value_name = "DIR")]
  pub dirs: Vec<String>,

  // The preopened mapped dir(s) to pass to wasmtime
  #[clap(long = "mapdir", short, value_name = "DIR:GUEST_PATH", parse(try_from_str = parse_map_dir))]
  pub mapdirs: Vec<(String, String)>,

  // A list of the allowed privileges on a particular folder/file
  #[clap(long = "fs-allow", value_name = "PATH:FLAGS", parse(try_from_str = parse_flag_str))]
  pub fs_allows: Vec<(String, BitFlags<AccessFs>)>,
}

pub fn get_args() -> Args {
  Args::parse()
}

fn parse_map_dir(s: &str) -> Result<(String, String)> {
  let parts: Vec<_> = s.splitn(2, ':').collect();
  if parts.len() != 2 {
    bail!("must be in the form 'DIR:GUEST_PATH'");
  }

  Ok((parts[0].to_string(), parts[1].to_string()))
}

fn parse_flag_str(s: &str) -> Result<(String, BitFlags<AccessFs>)> {
  let parts: Vec<_> = s.splitn(2, ':').collect();
  if parts.len() != 2 {
    bail!("must be in the form 'PATH:FLAGS'");
  }

  let path = parts[0].to_string();

  let mut flags: BitFlags<AccessFs> = BitFlags::EMPTY;
  for s in parts[1].split(',') {
    let f = parse_flag(s)?;
    flags = flags | f;
  }

  Ok((path, flags))
}

// roughly read = execute + read file + read dir
const ACCESS_FS_ROUGHLY_READ: BitFlags<AccessFs> =
  make_bitflags!(AccessFs::{Execute | ReadFile | ReadDir});
// roughly write = write file, remove dir/file, all the makes
const ACCESS_FS_ROUGHLY_WRITE: BitFlags<AccessFs> = make_bitflags!(AccessFs::{
    WriteFile | RemoveDir | RemoveFile | MakeChar | MakeDir | MakeReg | MakeSock
    | MakeFifo | MakeBlock | MakeSym});

fn parse_flag(s: &str) -> Result<BitFlags<AccessFs>> {
  match s {
    "*" => Ok(BitFlags::all()),
    "~read" => Ok(ACCESS_FS_ROUGHLY_READ),
    "~write" => Ok(ACCESS_FS_ROUGHLY_WRITE),
    _ => parse_single_flag(s).map(BitFlags::from),
  }
}

fn parse_single_flag(s: &str) -> Result<AccessFs> {
  // Placholder instead of the true landlock flags
  let f = match s {
    "X" => Execute,        // execute
    "W" => WriteFile,      // write file
    "R" => ReadFile,       // read file
    "RDir" => ReadDir,     // read dir
    "DDir" => RemoveDir,   // delete dir
    "D" => RemoveFile,     // delete file
    "MChar" => MakeChar,   // make char
    "MDir" => MakeDir,     // make dir
    "MReg" => MakeReg,     // make reg
    "MSock" => MakeSock,   // make sock
    "MFifo" => MakeFifo,   // make fifo
    "MBlock" => MakeBlock, // make block
    "MSym" => MakeSym,     // make symlink
    _ => bail!(format!("invalid flag provided: {}", s)),
  };

  Ok(f)
}
