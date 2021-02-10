use async_std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(setting(AppSettings::ColoredHelp))]
pub struct Cli {
    #[structopt(short = "n", long)]
    pub dry_run: bool,
    #[structopt(short, long)]
    pub run: bool,

    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    #[structopt(short = "T", long)]
    pub traverse_tree: bool,

    #[structopt(short, long)]
    pub file: bool,

    #[structopt(short, long)]
    pub directory: bool,

    #[structopt(short, long)]
    pub symlink: bool,

    #[structopt(short = "i", long)]
    pub case_insensetive: bool,

    pub search_pattern: String,
    pub replace_pattern: String,

    #[structopt(default_value = ".")]
    pub base_path: PathBuf,
}

impl Cli {
    pub fn use_env(&mut self) -> Result<(), String> {
        // checks and changes the running option according the environment varaiable
        let do_var_name = "FRS_DEFAULT_DO";
        if self.run && self.dry_run {
            return Err("run and dry-run flag specified".to_string());
        }
        match std::env::var(do_var_name)
            .unwrap_or("dry-run".to_string())
            .as_str()
        {
            "run" => {
                if !self.dry_run {
                    self.run = true
                }
            }
            "dry-run" => {
                if !self.dry_run {
                    self.dry_run = true
                }
            }
            invalid_do => {
                return Err(format!(
                    "Unknown content '{}' of environment varaiable '{}'",
                    invalid_do, do_var_name
                )
                .to_string())
            }
        }

        // dry-run sets automatically a minimal verbosity of one
        self.verbose = self.verbose.max(self.dry_run as u8);

        // if no type is selected, all are selected
        let no_type_selected = !(self.file || self.directory || self.symlink);
        self.file |= no_type_selected;
        self.directory |= no_type_selected;
        self.symlink |= no_type_selected;

        Ok(())
    }
}
