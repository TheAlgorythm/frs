use crate::cli::Cli;
use crate::fs::RenameInfo;
use cli_table::{
    format::{Border, Justify, Separator},
    print_stdout, Cell as TableCell, Table,
};
use colored::Colorize;
use std::cell::Cell;

#[cfg(test)]
#[path = "./stats_test.rs"]
pub mod stats_test;

pub struct Stats {
    show_renames: bool,
    show_summary: bool,
    operation_mode: String,
    base_path: String,
    errors: Cell<u32>,
    renamed_files: Cell<u32>,
    renamed_directories: Cell<u32>,
    renamed_symlinks: Cell<u32>,
    rename_arrow: String,
    error_icon: String,
    file_icon: String,
    dir_icon: String,
    symlink_icon: String,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            show_renames: false,
            show_summary: false,
            operation_mode: String::new(),
            base_path: String::new(),
            errors: Cell::new(0),
            renamed_files: Cell::new(0),
            renamed_directories: Cell::new(0),
            renamed_symlinks: Cell::new(0),
            rename_arrow: "=>".to_string(),
            error_icon: String::new(),
            file_icon: String::new(),
            dir_icon: String::new(),
            symlink_icon: String::new(),
        }
    }

    pub fn set_cli_opts(&mut self, opts: &Cli) {
        self.show_renames = opts.verbose >= 2;
        self.show_summary = opts.verbose >= 1;
        self.base_path = opts.base_path.to_string_lossy().to_string();
        if opts.run {
            self.operation_mode = "RUN".to_string();
        } else if opts.dry_run {
            self.operation_mode = "DRY-RUN".to_string();
        }
        if opts.icons {
            self.rename_arrow = "\u{21d2}".to_string();
            self.error_icon = "\u{f00d} ".to_string();
            self.file_icon = "\u{f15b} ".to_string();
            self.dir_icon = "\u{f07c} ".to_string();
            self.symlink_icon = "\u{f481} ".to_string();
        }
    }

    pub fn error(&self, error: &dyn std::fmt::Display) {
        eprintln!("{} {}!", "Error:".bright_red(), error);
        if self.show_summary {
            self.errors.set(self.errors.get() + 1);
        }
    }

    pub fn rename(&self, rename_info: &RenameInfo) {
        if self.show_renames {
            println!(
                "{} {} {}",
                rename_info.old_file.path.to_string_lossy().red(),
                self.rename_arrow.blue(),
                rename_info.new_path.to_string_lossy().green()
            );
        }
        if self.show_summary {
            if rename_info.old_file.file_type.is_file() {
                self.renamed_files.set(self.renamed_files.get() + 1);
            } else if rename_info.old_file.file_type.is_dir() {
                self.renamed_directories
                    .set(self.renamed_directories.get() + 1);
            } else if rename_info.old_file.file_type.is_symlink() {
                self.renamed_symlinks.set(self.renamed_symlinks.get() + 1);
            }
        }
    }

    fn has_output(&self) -> bool {
        self.errors.get() != 0
            || (self.show_renames
                && (self.renamed_files.get()
                    + self.renamed_directories.get()
                    + self.renamed_symlinks.get())
                    != 0)
    }

    pub fn print_summary(&self) {
        if !self.show_summary {
            return;
        }

        let mut num_formater = human_format::Formatter::new();
        let num_formater = num_formater.with_decimals(0).with_separator("");

        if self.has_output() {
            println!();
        }

        println!(
            "{} for {} ({}):",
            "Results".bold().bright_magenta(),
            self.base_path.underline().italic(),
            self.operation_mode
        );

        let mut infos = Vec::new();
        if self.errors.get() != 0 {
            infos.push(vec![
                format!("{}Errors", self.error_icon.bright_red()).cell(),
                num_formater
                    .format(self.errors.get() as f64)
                    .bright_red()
                    .cell()
                    .justify(Justify::Right),
            ]);
        }
        if self.renamed_files.get() != 0 {
            infos.push(vec![
                format!("{}Files", self.file_icon.bright_yellow()).cell(),
                num_formater
                    .format(self.renamed_files.get() as f64)
                    .bright_yellow()
                    .cell()
                    .justify(Justify::Right),
            ]);
        }
        if self.renamed_directories.get() != 0 {
            infos.push(vec![
                format!("{}Directories", self.dir_icon.blue()).cell(),
                num_formater
                    .format(self.renamed_directories.get() as f64)
                    .blue()
                    .cell()
                    .justify(Justify::Right),
            ]);
        }
        if self.renamed_symlinks.get() != 0 {
            infos.push(vec![
                format!("{}Symlinks", self.symlink_icon.yellow()).cell(),
                num_formater
                    .format(self.renamed_symlinks.get() as f64)
                    .yellow()
                    .cell()
                    .justify(Justify::Right),
            ]);
        }
        if infos.is_empty() {
            infos.push(vec!["Actions".cell(), 0.cell()]);
        }
        let info_table = infos
            .table()
            .border(Border::builder().build())
            .separator(Separator::builder().build());

        if let Err(error) = print_stdout(info_table) {
            self.error(&error);
        }
    }
}
