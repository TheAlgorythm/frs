use async_std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(setting(AppSettings::ColoredHelp))]
pub struct Cli {
    /// This is the default and lets you run it without the actual operation
    #[structopt(short = "n", long)]
    pub dry_run: bool,
    /// Actually running the rename operation.
    /// If you want to set this as default, set the environment variable `FRS_DEFAULT_OP` to `RUN`
    #[structopt(short, long)]
    pub run: bool,

    /// Set the verbosity. In a dry-run its automatically set to 1
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    #[structopt(short, long)]
    pub continue_on_error: bool,

    /// TODO
    #[structopt(short = "T", long)]
    pub traverse_tree: bool,

    /// Rename all matching files. If no type is set, then everything will be renamed
    #[structopt(short, long)]
    pub file: bool,

    /// Rename all matching directories. If no type is set, then everything will be renamed
    #[structopt(short, long)]
    pub directory: bool,

    /// Rename all matching symlinks. If no type is set, then everything will be renamed
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
    /// does all the automations after clap
    pub fn post_automations(&mut self) -> Result<(), String> {
        self.set_operation_mode()?;
        self.set_verbosity();
        self.set_types();
        Ok(())
    }

    /// checks and changes the running option according the environment varaiable
    fn set_operation_mode(&mut self) -> Result<(), String> {
        let do_var_name = "FRS_DEFAULT_OP";
        if self.run && self.dry_run {
            return Err("run and dry-run flag specified".to_string());
        }
        match std::env::var(do_var_name)
            .unwrap_or("DRY-RUN".to_string())
            .to_uppercase()
            .as_str()
        {
            "RUN" => {
                if !self.dry_run {
                    self.run = true
                }
            }
            "DRY-RUN" => {
                if !self.run {
                    self.dry_run = true
                }
            }
            invalid_do => {
                return Err(format!(
                    "Unknown content `{}` of environment varaiable `{}`",
                    invalid_do, do_var_name
                )
                .to_string())
            }
        }
        Ok(())
    }

    /// dry-run sets automatically a minimal verbosity of one
    fn set_verbosity(&mut self) {
        self.verbose = self.verbose.max(self.dry_run as u8);
    }

    /// if no type is selected, all are selected
    fn set_types(&mut self) {
        let no_type_selected = !(self.file || self.directory || self.symlink);
        self.file |= no_type_selected;
        self.directory |= no_type_selected;
        self.symlink |= no_type_selected;
    }
}
