# A WASM runtime with Landlock support

This is a proof-of-concept WebAssembly runtime that allows the definition of
access rule through the [Landlock](https://landlock.io) LSM.

## Requirements

* [Rust](https://www.rust-lang.org)
* A Linux distro with Landlock enabled (available since 5.13, [Arch](https://archlinux.org) should have it enabled by default)

## Command line usage

```
cargo run BIN [--dir DIR] [--mapdir OLD:NEW] [--fs-allow PATH:ACCESS] [--no-landlock]
```

Here `BIN` is the path to the WebAssembly binary (`.wasm`) and the only required argument.
The other options are explained in the table. All of them can be repeated except for `--no-landlock`.

| Argument                 | Description                                                                            |
| ------------------------ | -------------------------------------------------------------------------------------- |
| `--dir DIR`              | Preopens a directory `DIR` so that it can be accessed by the WASM binary               |
| `--mapdir OLD:NEW`       | Preopens a directory `OLD`, and the WASM binary will see it as if it were `NEW`        |
| `--fs-allow PATH:ACCESS` | Sets a list of comma-separated Landlock flags `ACCESS` for permitted actions on `PATH` |
| `--no-landlock`          | Disables Landlock                                                                      |

## Available access flags

For more details see [here](https://docs.kernel.org/userspace-api/landlock.html#filesystem-flags).

| Flag     | Meaning                                    |
| -------- | ------------------------------------------ |
| `X`      | Execute a file                             |
| `W`      | Write to a file                            |
| `R`      | Read a file                                |
| `RDir`   | Open a directory or list its content       |
| `DDir`   | Delete an empty directory or rename one    |
| `D`      | Unlink or rename a file                    |
| `MChar`  | Create, rename or link a character device  |
| `MDir`   | Create or rename a directory               |
| `MReg`   | Create, rename or link a regular file      |
| `MSock`  | Create, rename or link a socket            |
| `MFifo`  | Create, rename or link a named pipe        |
| `MBlock` | Create, rename or link a block device      |
| `MSym`   | Create, rename or link a symbolic link     |
| `read`   | Combination of `X`, `R` and `RDir`         |
| `write`  | Combination of all but `X`, `R` and `RDir` |
| `*`      | Enable all flags                           |