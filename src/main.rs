#![feature(async_closure)]
#![warn(clippy::use_self)]

#[cfg(test)]
#[macro_use]
extern crate matches;

#[cfg(test)]
#[macro_use]
extern crate maplit;

mod cli;
mod fs;
mod replace;

use async_std::sync::Arc;
use colored::*;
use structopt::StructOpt;

#[async_std::main]
async fn main() {
    let mut cli_opts = cli::Cli::from_args();
    if let Err(error) = cli_opts.post_automations() {
        println!("{} {}!", "Error:".bright_red(), error);
        return;
    }
    let replacer = match replace::Replacer::new(&cli_opts) {
        Ok(replacer) => Arc::new(replacer),
        Err(error) => {
            println!("{} {}!", "Error:".bright_red(), error);
            return;
        }
    };

    if let Err(error) = fs::rename(&cli_opts, replacer).await {
        println!("{} {}!", "Error:".bright_red(), error);
        return;
    }
}
