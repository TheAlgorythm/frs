#![feature(async_closure)]

mod cli;
mod replace;
mod fs;

use structopt::StructOpt;
use colored::*;

#[async_std::main]
async fn main() {
    let mut cli_opts = cli::Cli::from_args();
    if let Err(error) = cli_opts.use_env() {
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

    fs::rename(&cli_opts, &replacer).await;
}
