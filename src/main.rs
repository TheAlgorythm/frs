#![forbid(unsafe_code)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::clone_on_ref_ptr)]

use frs::{fs, Cli, Replacer, Stats};
use structopt::StructOpt;

#[async_std::main]
async fn main() {
    let mut stats = Stats::new();

    let mut cli_opts = Cli::from_args();
    if let Err(error) = cli_opts.post_automations() {
        stats.error(&error);
        return;
    }

    stats.set_cli_opts(&cli_opts);

    let replacer = match Replacer::new(&cli_opts) {
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

    stats.print_summary();
}
