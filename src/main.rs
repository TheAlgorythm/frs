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

mod cli;
mod fs;
mod replace;
mod utils;

use colored::Colorize;
use structopt::StructOpt;

#[async_std::main]
async fn main() {
    let mut cli_opts = cli::Cli::from_args();
    if let Err(error) = cli_opts.post_automations() {
        println!("{} {}!", "Error:".bright_red(), error);
        return;
    }
    let replacer = match replace::Replacer::new(&cli_opts) {
        Ok(replacer) => replacer,
        Err(error) => {
            println!("{} {}!", "Error:".bright_red(), error);
            return;
        }
    };

    if let Err(error) = fs::rename(&cli_opts, &replacer).await {
        println!("{} {}!", "Error:".bright_red(), error);
        return;
    }
}
