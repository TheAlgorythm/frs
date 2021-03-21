use super::*;
use crate::cli::cli_test::empty_cli;
use crate::fs::{FileInfo, RenameInfo};
use async_std::path::PathBuf;

#[test]
fn cli_0_verbose() {
    let cli_opts = empty_cli();

    let mut stats = Stats::new();
    stats.set_cli_opts(&cli_opts);

    assert!(!stats.show_renames);
    assert!(!stats.show_summary);
}

#[test]
fn cli_1_verbose() {
    let mut cli_opts = empty_cli();
    cli_opts.verbose = 1;

    let mut stats = Stats::new();
    stats.set_cli_opts(&cli_opts);

    assert!(!stats.show_renames);
    assert!(stats.show_summary);
}

#[test]
fn cli_2_verbose() {
    let mut cli_opts = empty_cli();
    cli_opts.verbose = 2;

    let mut stats = Stats::new();
    stats.set_cli_opts(&cli_opts);

    assert!(stats.show_renames);
    assert!(stats.show_summary);
}

#[test]
fn not_count_error_without_summary() {
    let stats = Stats::new();

    stats.error(&String::new());

    assert_eq!(stats.errors.get(), 0);
    assert_eq!(stats.renamed_files.get(), 0);
    assert_eq!(stats.renamed_directories.get(), 0);
    assert_eq!(stats.renamed_symlinks.get(), 0);
}

#[test]
fn count_error_with_summary() {
    let mut stats = Stats::new();
    stats.show_summary = true;

    stats.error(&String::new());

    assert_eq!(stats.errors.get(), 1);
    assert_eq!(stats.renamed_files.get(), 0);
    assert_eq!(stats.renamed_directories.get(), 0);
    assert_eq!(stats.renamed_symlinks.get(), 0);
}

#[test]
fn not_count_file_without_summary() {
    let stats = Stats::new();

    stats.rename(&RenameInfo {
        old_file: FileInfo::file(PathBuf::new()),
        new_path: PathBuf::new(),
    });

    assert_eq!(stats.errors.get(), 0);
    assert_eq!(stats.renamed_files.get(), 0);
    assert_eq!(stats.renamed_directories.get(), 0);
    assert_eq!(stats.renamed_symlinks.get(), 0);
}

#[test]
fn count_file_with_summary() {
    let mut stats = Stats::new();
    stats.show_summary = true;

    stats.rename(&RenameInfo {
        old_file: FileInfo::file(PathBuf::new()),
        new_path: PathBuf::new(),
    });

    assert_eq!(stats.errors.get(), 0);
    assert_eq!(stats.renamed_files.get(), 1);
    assert_eq!(stats.renamed_directories.get(), 0);
    assert_eq!(stats.renamed_symlinks.get(), 0);
}

#[test]
fn has_no_output_as_empty() {
    let mut stats = Stats::new();
    stats.show_renames = true;

    assert!(!stats.has_output());
}

#[test]
fn has_no_output_as_dont_show_renames() {
    let stats = Stats::new();
    stats.renamed_files.set(42);

    assert!(!stats.has_output());
}

#[test]
fn has_output_as_error() {
    let stats = Stats::new();
    stats.errors.set(42);

    assert!(stats.has_output());
}

#[test]
fn has_output_as_renames() {
    let mut stats = Stats::new();
    stats.show_renames = true;
    stats.renamed_files.set(42);

    assert!(stats.has_output());
}
