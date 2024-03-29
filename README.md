# File Rename Search

![MIT License](https://img.shields.io/github/license/TheAlgorythm/frs?style=for-the-badge&logo=open-source-initiative)
[![Workflow Status](https://img.shields.io/github/actions/workflow/status/TheAlgorythm/frs/check.yml?branch=MAIN&style=for-the-badge)](https://github.com/TheAlgorythm/frs/actions?query=workflow%3ARust)
[![Crates.io](https://img.shields.io/crates/v/frs?style=for-the-badge&logo=rust)](https://crates.io/crates/frs)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/frs.svg)](https://web.crev.dev/rust-reviews/crate/frs/)

Rename files with RegEx patterns.

![Demo](https://zschoen.dev/img/frs_casts.svg)

## Usage

```zsh
$ frs --help

USAGE:
    frs [FLAGS] [OPTIONS] <search-pattern> <replace-pattern> [base-path]

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
    -T, --traverse-tree        This traverses the Directory Tree. If set, the renaming of directories will be disabled
                               by default, to prevent the renaming of a directory and its inner files
    -V, --version              Prints version information
    -v, --verbose              Set the verbosity. In a dry-run its automatically set to 1

OPTIONS:
    -I, --icons <icons>     [env: FRS_SHOW_ICONS=]  [default: true]

ARGS:
    <search-pattern>
    <replace-pattern>
    <base-path>           [default: .]
```

### Example

```zsh
$ frs -f '([a-z]+)_(\d+)(\..+)' '${2}_${1}${3}' test_folder
test_folder/bar_02.png -> test_folder/02_bar.png
test_folder/foo_01.txt -> test_folder/01_foo.txt
```

## Installation

### Cargo

Please check, that you have a recent version of Rust installed.

```zsh
$ cargo install frs
```

## Setup

### Operation Mode

The default operation is to do a dry-run. To change this behavior, you have to set the `FRS_DEFAULT_OP` environment variable to `RUN`.

### Icons

![Nerd Font Icons](https://zschoen.dev/img/frs_icons.png)

frs is able to show [Nerd Font](https://www.nerdfonts.com/) icons. If you don't use NF in your terminal you can disable it with the `--icons` option or with the `FRS_SHOW_ICONS` environment variable.
This can be done e.g. on ZSH by adding this line to your ~/.zshenv:

```zsh
export FRS_SHOW_ICONS=false
```


## CREV - Rust code reviews - Raise awareness

Please, spread this info !\
Open source code needs a community effort to express trustworthiness.\
Start with reading the reviews of the crates you use. Example: [web.crev.dev/rust-reviews/crate/num-traits/](https://web.crev.dev/rust-reviews/crate/num-traits/) \
Than install the CLI [cargo-crev](https://github.com/crev-dev/cargo-crev)\. Read the [Getting Started guide](https://github.com/crev-dev/cargo-crev/blob/master/cargo-crev/src/doc/getting_started.md). \
On your Rust project, verify the trustworthiness of all dependencies, including transient dependencies with `cargo crev verify`\
Write a new review ! \
Describe the crates you trust. Or warn about the crate versions you think are dangerous.\
Help other developers, inform them and share your opinion.\
Use the helper on this webpage: [web.crev.dev/rust-reviews/review_new](https://web.crev.dev/rust-reviews/review_new)

