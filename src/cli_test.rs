use super::*;

pub fn empty_cli() -> Cli {
    Cli {
        run: false,
        dry_run: false,
        continue_on_error: false,
        case_insensetive: false,
        file: false,
        directory: false,
        symlink: false,
        traverse_tree: false,
        verbose: 0,
        search_pattern: String::new(),
        replace_pattern: String::new(),
        base_path: PathBuf::new(),
    }
}

#[test]
fn multiple_operation_modes() {
    let mut cli = empty_cli();
    cli.run = true;
    cli.dry_run = true;

    assert_matches!(cli.set_operation_mode(), Err(Error::MultipleOperationModes));
}

#[test]
fn invalid_env_var() {
    let mut cli = empty_cli();

    std::env::set_var("FRS_DEFAULT_OP", "invalid");
    assert_matches!(
        cli.set_operation_mode(),
        Err(Error::UnknownEnvVarContent { .. })
    );
}

#[test]
fn default_operation_run() {
    let mut cli = empty_cli();
    std::env::set_var("FRS_DEFAULT_OP", "run");

    assert_matches!(cli.set_operation_mode(), Ok(()));
    assert!(cli.run);
    assert!(!cli.dry_run);
}

#[test]
fn default_operation_run_with_run() {
    let mut cli = empty_cli();
    cli.run = true;
    std::env::set_var("FRS_DEFAULT_OP", "run");

    assert_matches!(cli.set_operation_mode(), Ok(()));
    assert!(cli.run);
    assert!(!cli.dry_run);
}

#[test]
fn default_operation_run_with_dry_run() {
    let mut cli = empty_cli();
    cli.dry_run = true;
    std::env::set_var("FRS_DEFAULT_OP", "run");

    assert_matches!(cli.set_operation_mode(), Ok(()));
    assert!(!cli.run);
    assert!(cli.dry_run);
}

#[test]
fn default_operation_dry_run() {
    let mut cli = empty_cli();
    std::env::set_var("FRS_DEFAULT_OP", "dry-run");

    assert_matches!(cli.set_operation_mode(), Ok(()));
    assert!(!cli.run);
    assert!(cli.dry_run);
}

#[test]
fn default_operation_dry_run_with_run() {
    let mut cli = empty_cli();
    cli.run = true;
    std::env::set_var("FRS_DEFAULT_OP", "dry-run");

    assert_matches!(cli.set_operation_mode(), Ok(()));
    assert!(cli.run);
    assert!(!cli.dry_run);
}

#[test]
fn default_operation_dry_run_with_dry_run() {
    let mut cli = empty_cli();
    cli.dry_run = true;
    std::env::set_var("FRS_DEFAULT_OP", "dry-run");

    assert_matches!(cli.set_operation_mode(), Ok(()));
    assert!(!cli.run);
    assert!(cli.dry_run);
}

#[test]
fn low_verbosity_on_run() {
    let mut cli = empty_cli();

    cli.set_verbosity();
    assert_eq!(cli.verbose, 0);
}

#[test]
fn high_verbosity_on_run() {
    let mut cli = empty_cli();
    cli.verbose = 10;

    cli.set_verbosity();
    assert_eq!(cli.verbose, 10);
}

#[test]
fn low_verbosity_on_dry_run() {
    let mut cli = empty_cli();
    cli.dry_run = true;

    cli.set_verbosity();
    assert_eq!(cli.verbose, 1);
}

#[test]
fn high_verbosity_on_dry_run() {
    let mut cli = empty_cli();
    cli.dry_run = true;
    cli.verbose = 10;

    cli.set_verbosity();
    assert_eq!(cli.verbose, 10);
}

#[test]
fn no_filetype_set() {
    let mut cli = empty_cli();

    cli.set_types();
    assert!(cli.file);
    assert!(cli.directory);
    assert!(cli.symlink);
}

#[test]
fn all_filetypes_set() {
    let mut cli = empty_cli();
    cli.file = true;
    cli.directory = true;
    cli.symlink = true;

    cli.set_types();
    assert!(cli.file);
    assert!(cli.directory);
    assert!(cli.symlink);
}

#[test]
fn file_filetype_set() {
    let mut cli = empty_cli();
    cli.file = true;

    cli.set_types();
    assert!(cli.file);
    assert!(!cli.directory);
    assert!(!cli.symlink);
}

#[test]
fn not_file_filetypes_set() {
    let mut cli = empty_cli();
    cli.directory = true;
    cli.symlink = true;

    cli.set_types();
    assert!(!cli.file);
    assert!(cli.directory);
    assert!(cli.symlink);
}

#[test]
fn no_filetype_set_with_traverse_tree() {
    let mut cli = empty_cli();
    cli.traverse_tree = true;

    cli.set_types();
    assert!(cli.file);
    assert!(!cli.directory);
    assert!(cli.symlink);
}

#[test]
fn all_filetypes_set_with_traverse_tree() {
    let mut cli = empty_cli();
    cli.traverse_tree = true;
    cli.file = true;
    cli.directory = true;
    cli.symlink = true;

    cli.set_types();
    assert!(cli.file);
    assert!(cli.directory);
    assert!(cli.symlink);
}

#[test]
fn dir_filetype_set_with_traverse_tree() {
    let mut cli = empty_cli();
    cli.traverse_tree = true;
    cli.directory = true;

    cli.set_types();
    assert!(!cli.file);
    assert!(cli.directory);
    assert!(!cli.symlink);
}

#[test]
fn not_dir_filetypes_set_with_traverse_tree() {
    let mut cli = empty_cli();
    cli.traverse_tree = true;
    cli.file = true;
    cli.symlink = true;

    cli.set_types();
    assert!(cli.file);
    assert!(!cli.directory);
    assert!(cli.symlink);
}
