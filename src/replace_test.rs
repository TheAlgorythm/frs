use super::*;
use crate::cli::cli_test::empty_cli;
use std::ffi::OsStr;

pub fn empty_replacer() -> Replacer {
    Replacer {
        search: Regex::new("(.+)").unwrap(),
        replace_pattern: "${1}".to_string(),
    }
}

pub fn restrictive_replacer() -> Replacer {
    Replacer {
        search: Regex::new("_(.+)").unwrap(),
        replace_pattern: "${1}".to_string(),
    }
}

#[test]
fn valid_regex() {
    let mut cli = empty_cli();
    cli.search_pattern = "(.+)".to_string();
    cli.replace_pattern = "${1}".to_string();

    assert_matches!(Replacer::new(&cli), Ok(_));
}

#[test]
fn invalid_regex() {
    let mut cli = empty_cli();
    cli.search_pattern = "(.+".to_string();
    cli.replace_pattern = "${1}".to_string();

    assert_matches!(Replacer::new(&cli), Err(_));
}

#[test]
fn match_matching_filename() {
    let replacer = restrictive_replacer();

    assert_matches!(replacer.is_match(Path::new("dir/_test")), Ok(true));
}

#[test]
fn match_non_matching_filenames() {
    let replacer = restrictive_replacer();

    assert_matches!(replacer.is_match(Path::new("dir/test")), Ok(false));
}

#[test]
fn match_invalid_filenames() {
    let replacer = empty_replacer();

    assert_matches!(
        replacer.is_match(Path::new("..")),
        Err(Error::InvalidFileName(_))
    );
    assert_matches!(
        replacer.is_match(Path::new(".")),
        Err(Error::InvalidFileName(_))
    );
    assert_matches!(
        replacer.is_match(Path::new("/")),
        Err(Error::InvalidFileName(_))
    );
}

#[cfg(any(unix, target_os = "redox"))]
#[test]
fn match_non_utf8_filenames() {
    use std::os::unix::ffi::OsStrExt;
    let replacer = empty_replacer();

    assert_matches!(
        replacer.is_match(Path::new(OsStr::from_bytes(&[0x66, 0x6f, 0x80, 0x6f]))),
        Err(Error::Utf8Invalid(_))
    );
}

#[cfg(windows)]
#[test]
fn match_non_utf8_filenames() {
    use std::os::windows::prelude::*;
    let replacer = empty_replacer();

    assert_matches!(
        replacer.is_match(Path::new(OsStr::from_bytes(&[
            0x0066, 0x006f, 0xD800, 0x006f
        ]))),
        Err(Error::Utf8Invalid(_))
    );
}

#[test]
fn replace_matching_filenames() {
    let replacer = restrictive_replacer();

    assert_eq!(
        replacer.replace(Path::new("../_test")).unwrap(),
        PathBuf::from("../test")
    );
    assert_eq!(
        replacer.replace(Path::new("/_foo")).unwrap(),
        PathBuf::from("/foo")
    );
    assert_eq!(
        replacer.replace(Path::new("/_foo/_bar")).unwrap(),
        PathBuf::from("/_foo/bar")
    );
}

#[test]
fn replace_non_matching_filenames() {
    let replacer = restrictive_replacer();

    assert_eq!(
        replacer.replace(Path::new("../test")).unwrap(),
        PathBuf::from("../test")
    );
    assert_eq!(
        replacer.replace(Path::new("/foo")).unwrap(),
        PathBuf::from("/foo")
    );
    assert_eq!(
        replacer.replace(Path::new("/_foo/bar")).unwrap(),
        PathBuf::from("/_foo/bar")
    );
}

#[test]
fn replace_invalid_filenames() {
    let replacer = empty_replacer();

    assert_matches!(
        replacer.replace(Path::new("..")),
        Err(Error::InvalidFileName(_))
    );
    assert_matches!(
        replacer.replace(Path::new(".")),
        Err(Error::InvalidFileName(_))
    );
}

#[test]
fn replace_no_parent() {
    let replacer = empty_replacer();

    assert_matches!(replacer.replace(Path::new("/")), Err(Error::NoParent(_)));
}

#[cfg(any(unix, target_os = "redox"))]
#[test]
fn replace_non_utf8_filenames() {
    use std::os::unix::ffi::OsStrExt;
    let replacer = empty_replacer();

    assert_matches!(
        replacer.replace(Path::new(OsStr::from_bytes(&[0x66, 0x6f, 0x80, 0x6f]))),
        Err(Error::Utf8Invalid(_))
    );
}

#[cfg(windows)]
#[test]
fn replace_non_utf8_filenames() {
    use std::os::windows::prelude::*;
    let replacer = empty_replacer();

    assert_matches!(
        replacer.replace(Path::new(OsStr::from_bytes(&[
            0x0066, 0x006f, 0xD800, 0x006f
        ]))),
        Err(Error::Utf8Invalid(_))
    );
}
