# File Rename Search ![Rust](https://img.shields.io/badge/Rust-frs-success?style=for-the-badge&logo=rust)

![MIT License](https://img.shields.io/github/license/TheAlgorythm/frs?style=for-the-badge&logo=open-source-initiative)
[![Workflow Status](https://img.shields.io/github/workflow/status/TheAlgorythm/frs/Rust?style=for-the-badge)](https://github.com/TheAlgorythm/frs/actions?query=workflow%3ARust)
[![Crates.io](https://img.shields.io/crates/v/frs?style=for-the-badge)](https://crates.io/crates/frs)

Rename files with RegEx patterns.

## Usage

```zsh
$ frs --help
frs 0.1.0

USAGE:
  frs [FLAGS] <search-pattern> <replace-pattern> [base-path]

FLAGS:
  -i, --case-insensetive
  -c, --continue-on-error
  -d, --directory            Rename all matching directories. If no type is set, then everything will be renamed
  -n, --dry-run              This is the default and lets you run it without the actual operation
  -f, --file                 Rename all matching files. If no type is set, then everything will be renamed
  -h, --help                 Prints help information
  -r, --run                  Actually running the rename operation. If you want to set this as default, set the
                            environment variable `FRS_DEFAULT_OP` to `RUN`
  -s, --symlink              Rename all matching symlinks. If no type is set, then everything will be renamed
  -T, --traverse-tree        TODO
  -V, --version              Prints version information
  -v, --verbose              Set the verbosity. In a dry-run its automatically set to 1

ARGS:
  <search-pattern>
  <replace-pattern>
  <base-path>           [default: .]
```

## Installation

### Cargo

Please check, that you have a recent version of Rust installed.

```zsh
$ cargo +nightly install frs
```

## Setup

The default operation is to do a dry-run. To change this behavior, you have to set the `FRS_DEFAULT_OP` environment variable to `RUN`.

