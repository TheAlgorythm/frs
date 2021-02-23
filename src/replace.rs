use async_std::path::{Path, PathBuf};
use regex::{Regex, RegexBuilder};
use std::fmt;

#[cfg(test)]
#[path = "./replace_test.rs"]
pub mod replace_test;

#[derive(Debug)]
pub enum Error {
    InvalidFileName(PathBuf),
    NoParent(PathBuf),
    Utf8Invalid(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidFileName(ref path) => {
                write!(f, "The filename `{}`  is invalid", path.to_string_lossy())
            }
            Error::NoParent(ref path) => {
                write!(f, "There is no parent of `{}`", path.to_string_lossy())
            }
            Error::Utf8Invalid(ref path) => write!(
                f,
                "There is a conversion error in `{}` to UTF-8",
                path.to_string_lossy()
            ),
        }
    }
}

pub struct Replacer {
    search: Regex,
    replace_pattern: String,
}

impl Replacer {
    pub fn new(opts: &super::cli::Cli) -> Result<Self, regex::Error> {
        Ok(Replacer {
            search: RegexBuilder::new(&opts.search_pattern.clone())
                .case_insensitive(opts.case_insensetive)
                .build()?,
            replace_pattern: opts.replace_pattern.clone(),
        })
    }

    pub fn is_match(&self, file: &Path) -> Result<bool, Error> {
        Ok(self.search.is_match(
            file.file_name()
                .ok_or_else(|| Error::InvalidFileName(file.to_path_buf()))?
                .to_str()
                .ok_or_else(|| Error::Utf8Invalid(PathBuf::from(file.file_name().unwrap())))?,
        ))
    }

    pub fn replace(&self, file: &Path) -> Result<PathBuf, Error> {
        let mut new_path = file
            .parent()
            .ok_or_else(|| Error::NoParent(file.to_path_buf()))?
            .to_path_buf();
        new_path.push(
            self.search
                .replace(
                    file.file_name()
                        .ok_or_else(|| Error::InvalidFileName(file.to_path_buf()))?
                        .to_str()
                        .ok_or_else(|| {
                            Error::Utf8Invalid(PathBuf::from(file.file_name().unwrap()))
                        })?,
                    self.replace_pattern.as_str(),
                )
                .into_owned(),
        );
        Ok(new_path)
    }
}
