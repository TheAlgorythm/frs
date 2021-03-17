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
mod utils;

mod cli;
mod stats;
mod fs;
mod replace;

use structopt::StructOpt;

#[async_std::main]
async fn main() {
    let mut stats = stats::Stats::new();

    let mut cli_opts = cli::Cli::from_args();
    if let Err(error) = cli_opts.post_automations() {
        stats.error(&error);
        return;
    }

    stats.set_cli_opts(&cli_opts);

    let replacer = match replace::Replacer::new(&cli_opts) {
        Ok(replacer) => replacer,
        Err(error) => {
            stats.error(&error);
            return;
        }
    };

    if let Err(error) = fs::rename(&cli_opts, &replacer, &stats).await {
        stats.error(&error);
        return;
    }

    stats.print();
}
