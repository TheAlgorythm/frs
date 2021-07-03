use crate::cli::Cli;
use crate::fs::RenameInfo;
use cli_table::{
    format::{Border, Justify, Separator},
    print_stdout, Cell as TableCell, Table,
};
use colored::Colorize;
use std::cell::Cell;
use terminal_size::{terminal_size, Width};

#[cfg(test)]
#[path = "./stats_test.rs"]
pub mod stats_test;

macro_rules! add_info {
    ($infos:ident, $num_formater:ident, $info_name:literal, $elem:expr, $icon:expr, $color:ident) => {
        let count = $elem.get();
        if count != 0 {
            $infos.push(vec![
                format!("{}{}", $icon.$color(), $info_name).cell(),
                $num_formater
                    .format(count as f64)
                    .$color()
                    .cell()
                    .justify(Justify::Right),
            ]);
        }
    };
}

pub struct Stats {
    show_renames: bool,
    show_summary: bool,
    operation_mode: String,
    base_path: String,
    errors: Cell<u32>,
    renamed_files: Cell<u32>,
    renamed_directories: Cell<u32>,
    renamed_symlinks: Cell<u32>,
    middle_col: usize,
    max_indent: Cell<usize>,
    rename_arrow: String,
    error_icon: String,
    file_icon: String,
    dir_icon: String,
    symlink_icon: String,
}

impl Stats {
    pub fn new() -> Self {
        let middle_col =
            terminal_size().map_or(0, |(Width(w), _)| (w as usize / 2).saturating_sub(2));

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
            middle_col,
            max_indent: Cell::new(0),
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
            let old_path = rename_info.old_file.path.to_string_lossy();
            self.max_indent
                .set(self.max_indent.get().max(old_path.len()));
            println!(
                "{old_path} {empty:indent$}{arrow} {new_path}",
                old_path = old_path.red(),
                new_path = rename_info.new_path.to_string_lossy().green(),
                arrow = self.rename_arrow.blue(),
                empty = "",
                indent = self
                    .middle_col
                    .min(self.max_indent.get())
                    .saturating_sub(old_path.len()),
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

        add_info!(
            infos,
            num_formater,
            "Errors",
            self.errors,
            self.error_icon,
            bright_red
        );
        add_info!(
            infos,
            num_formater,
            "Files",
            self.renamed_files,
            self.file_icon,
            bright_yellow
        );
        add_info!(
            infos,
            num_formater,
            "Directories",
            self.renamed_directories,
            self.dir_icon,
            blue
        );
        add_info!(
            infos,
            num_formater,
            "Symlinks",
            self.renamed_symlinks,
            self.symlink_icon,
            yellow
        );

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
