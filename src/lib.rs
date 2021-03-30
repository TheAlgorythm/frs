#![forbid(unsafe_code)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::clone_on_ref_ptr)]

#[cfg(test)]
#[macro_use]
extern crate matches;

#[cfg(test)]
#[macro_use]
extern crate maplit;

#[macro_use]
pub mod utils;

mod cli;
mod fs;
mod replace;
mod stats;
