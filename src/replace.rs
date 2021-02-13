use async_std::path::{Path, PathBuf};
use regex::{Regex, RegexBuilder};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidFileName,
    NoParent,
    Utf8Invalid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidFileName => write!(f, "The filename is invalid"),
            Error::NoParent => write!(f, "There is no parent"),
            Error::Utf8Invalid => write!(f, "There is a conversion error to UTF-8"),
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
                .ok_or(Error::InvalidFileName)?
                .to_str()
                .ok_or(Error::Utf8Invalid)?,
        ))
    }

    pub fn replace(&self, file: &Path) -> Result<PathBuf, Error> {
        let mut new_path = file.parent().ok_or(Error::NoParent)?.to_path_buf();
        new_path.push(
            self.search
                .replace(
                    file.file_name()
                        .ok_or(Error::InvalidFileName)?
                        .to_str()
                        .ok_or(Error::Utf8Invalid)?,
                    self.replace_pattern.as_str(),
                )
                .into_owned(),
        );
        Ok(new_path)
    }
}
