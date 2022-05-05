use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  // The preopepend dir to pass to wasmtime
  #[clap(long, short)]
  pub preopened_dir: String,
  // A list of the allowed privileges on a particular folder/file
  #[clap(long = "fs-allow", number_of_values = 1, value_name = "PATH:FLAGS", parse(try_from_str = parse_flag_str))]
  pub fs_allows: Vec<(String, u64)>,

  // A list of the denied privileges on a particular folder/file
  #[clap(long = "fs-deny", number_of_values = 1, value_name = "PATH:FLAGS", parse(try_from_str = parse_flag_str))]
  pub fs_denies: Vec<(String, u64)>,
}

pub fn get_args() -> Args {
  Args::parse()
}

fn parse_flag_str(s: &str) -> Result<(String, u64)> {
  let parts: Vec<_> = s.splitn(2, ':').collect();
  if parts.len() != 2 {
    bail!("must be in the form 'PATH:FLAGS'");
  }

  let path = parts[0].to_owned();
  let mut flags: u64 = 0;

  for s in parts[1].split(',') {
    let f = parse_flag(s)?;
    flags = flags | f;
  }

  Ok((path, flags))
}

// roughly read = execute + read file + read dir
const ACCESS_FS_ROUGHLY_READ: u64 = 1 | 4 | 8;
// roughly write = write file, remove dir/file, all the makes
const ACCESS_FS_ROUGHLY_WRITE: u64 = 2 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048 | 4096;

fn parse_flag(s: &str) -> Result<u64> {
  // Placholder instead of the true landlock flags
  match s {
    "*" => Ok(ACCESS_FS_ROUGHLY_READ | ACCESS_FS_ROUGHLY_WRITE),
    "~read" => Ok(ACCESS_FS_ROUGHLY_READ),
    "~write" => Ok(ACCESS_FS_ROUGHLY_WRITE),
    "X" => Ok(1),         // execute
    "W" => Ok(2),         // write file
    "R" => Ok(4),         // read file
    "RDir" => Ok(8),      // read dir
    "DDir" => Ok(16),     // delete dir
    "D" => Ok(32),        // delete file
    "MChar" => Ok(64),    // make char
    "MDir" => Ok(128),    // make dir
    "MReg" => Ok(256),    // make reg
    "MSock" => Ok(512),   // make sock
    "MFifo" => Ok(1024),  // make fifo
    "MBlock" => Ok(2048), // make block
    "MSym" => Ok(4096),   // make symlink
    _ => bail!(format!("invalid flag provided: {}", s)),
  }
}
