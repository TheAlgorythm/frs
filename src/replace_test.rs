use super::*;
use std::ffi::OsStr;

#[test]
fn match_invalid_filenames() {
    let replacer = Replacer {
        search: Regex::new("(.+)").unwrap(),
        replace_pattern: "${1}".to_string(),
    };

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
    let replacer = Replacer {
        search: Regex::new("(.+)").unwrap(),
        replace_pattern: "${1}".to_string(),
    };

    assert_matches!(
        replacer.is_match(Path::new(OsStr::from_bytes(&[0x66, 0x6f, 0x80, 0x6f]))),
        Err(Error::Utf8Invalid(_))
    );
}

#[cfg(windows)]
#[test]
fn match_non_utf8_filenames() {
    use std::os::windows::prelude::*;
    let replacer = Replacer {
        search: Regex::new("(.+)").unwrap(),
        replace_pattern: "${1}".to_string(),
    };

    assert_matches!(
        replacer.is_match(Path::new(OsStr::from_bytes(&[
            0x0066, 0x006f, 0xD800, 0x006f
        ]))),
        Err(Error::Utf8Invalid(_))
    );
}

#[test]
fn replace_invalid_filenames() {
    let replacer = Replacer {
        search: Regex::new("(.+)").unwrap(),
        replace_pattern: "${1}".to_string(),
    };

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
    let replacer = Replacer {
        search: Regex::new("(.+)").unwrap(),
        replace_pattern: "${1}".to_string(),
    };

    assert_matches!(replacer.replace(Path::new("/")), Err(Error::NoParent(_)));
}

#[cfg(any(unix, target_os = "redox"))]
#[test]
fn replace_non_utf8_filenames() {
    use std::os::unix::ffi::OsStrExt;
    let replacer = Replacer {
        search: Regex::new("(.+)").unwrap(),
        replace_pattern: "${1}".to_string(),
    };

    assert_matches!(
        replacer.replace(Path::new(OsStr::from_bytes(&[0x66, 0x6f, 0x80, 0x6f]))),
        Err(Error::Utf8Invalid(_))
    );
}

#[cfg(windows)]
#[test]
fn replace_non_utf8_filenames() {
    use std::os::windows::prelude::*;
    let replacer = Replacer {
        search: Regex::new("(.+)").unwrap(),
        replace_pattern: "${1}".to_string(),
    };

    assert_matches!(
        replacer.replace(Path::new(OsStr::from_bytes(&[
            0x0066, 0x006f, 0xD800, 0x006f
        ]))),
        Err(Error::Utf8Invalid(_))
    );
}
