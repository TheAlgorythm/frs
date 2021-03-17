use super::cli::Cli;
use async_std::path::PathBuf;
use colored::Colorize;
// use std::sync::atomic::{AtomicU32, Ordering};
use std::cell::Cell;

pub struct Stats {
    show_renames: bool,
    show_conclusion: bool,
    // errors: AtomicU32,
    errors: Cell<u32>,
    renamed_files: u32,
    renamed_directories: u32,
    renamed_symlinks: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            show_renames: false,
            show_conclusion: false,
            // errors: AtomicU32::new(0),
            errors: Cell::new(0),
            renamed_files: 0,
            renamed_directories: 0,
            renamed_symlinks: 0,
        }
    }

    pub fn set_cli_opts(&mut self, opts: &Cli) {
        self.show_renames = opts.verbose >= 1;
        self.show_conclusion = opts.verbose >= 2;
    }

    pub fn error(&self, error: &dyn std::fmt::Display) {
        eprintln!("{} {}!", "Error:".bright_red(), error);
        if self.show_conclusion {
            // self.errors.fetch_add(1, Ordering::SeqCst);
            self.errors.set(self.errors.get() + 1);
        }
    }

    pub fn rename(&self, from: &PathBuf, to: &PathBuf) {
        if self.show_renames {
            println!(
                "{} {} {}",
                from.to_string_lossy().red(),
                "->".blue(),
                to.to_string_lossy().green()
            );
        }
    }

    pub fn print(&self) {
        if !self.show_conclusion {
            return;
        }

        // println!("Errors: {}", self.errors.load(Ordering::Relaxed));
        println!("Errors: {}", self.errors.get());
    }
}
