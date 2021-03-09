use async_std::path::{Path, PathBuf};
use regex::{Regex, RegexBuilder};

#[cfg(test)]
#[path = "./replace_test.rs"]
pub mod replace_test;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The filename `{}` is invalid", .0.to_string_lossy())]
    InvalidFileName(PathBuf),
    #[error("There is no parent of `{}`", .0.to_string_lossy())]
    NoParent(PathBuf),
    #[error("There is a conversion error in `{}` to UTF-8", .0.to_string_lossy())]
    Utf8Invalid(PathBuf),
}

#[derive(Debug)]
pub struct Replacer {
    search: Regex,
    replace_pattern: String,
}

impl Replacer {
    pub fn new(opts: &super::cli::Cli) -> Result<Self, regex::Error> {
        Ok(Self {
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
