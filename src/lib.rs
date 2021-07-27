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

pub mod cli;
pub mod fs;
pub mod replace;
pub mod stats;

pub use cli::Cli;
pub use replace::Replacer;
pub use stats::Stats;
