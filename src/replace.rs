use regex::{Regex, RegexBuilder};
use async_std::path::{Path, PathBuf};

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

    pub fn is_match(&self, file: &Path) -> Option<bool> {
        Some(self.search.is_match(file.file_name()?.to_str()?))
    }

    pub fn replace(&self, file: &Path) -> Option<PathBuf> {
        let mut new_path = file.parent()?.to_path_buf();
        new_path.push(self.search.replace(file.file_name()?.to_str()?, self.replace_pattern.as_str()).into_owned());
        Some(new_path)
    }
}
