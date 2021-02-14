use super::replace;
use async_std::{fs, future, io, path::PathBuf};
use colored::*;
use futures::stream::{StreamExt, TryStreamExt};
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;
use std::sync::RwLock;

pub enum Error {
    Io(io::Error),
    Replace(replace::Error),
    NonExistingParent(PathBuf),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<replace::Error> for Error {
    fn from(err: replace::Error) -> Self {
        Error::Replace(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Replace(ref err) => err.fmt(f),
            Error::NonExistingParent(ref parent) => write!(
                f,
                "The parent directory `{}` does not exist",
                parent.to_string_lossy()
            ),
        }
    }
}

pub async fn rename(opts: &super::cli::Cli, replacer: &replace::Replacer) -> Result<(), Error> {
    let targets = Rc::new(RwLock::new(HashSet::new()));
    fs::read_dir(opts.base_path.clone())
        .await?
        .filter_map(async move |file_path| {
            let file_path = match file_path {
                Ok(file_path) => file_path,
                Err(error) => return Some(Err(Error::from(error))),
            };
            let file_type = match file_path.file_type().await {
                Ok(file_type) => file_type,
                Err(error) => return Some(Err(Error::from(error))),
            };
            if (file_type.is_file() && opts.file)
                || (file_type.is_dir() && opts.directory)
                || (file_type.is_symlink() && opts.symlink)
            {
                return Some(Ok(file_path.path()));
            }
            None
        })
        .try_filter(|file_path| {
            future::ready(
                !targets
                    .read()
                    .expect(
                        format!(
                            "{} Poisoned sync-lock on read!",
                            "Fatal Error:".bright_red()
                        )
                        .as_str(),
                    )
                    .contains(file_path)
                    && replacer.is_match(file_path).unwrap_or(true),
            )
        })
        .map_ok(async move |file_path| {
            let new_file_path = replacer.replace(&file_path)?;
            if !new_file_path
                .parent()
                .expect("Couldn't get parent!")
                .is_dir()
                .await
            {
                return Err(Error::NonExistingParent(
                    new_file_path
                        .parent()
                        .expect("Couldn't get parent!")
                        .to_path_buf(),
                ));
            }
            Ok((file_path.clone(), new_file_path))
        })
        .try_for_each(|file_paths| {
            let targets = targets.clone();
            async move {
                let (old_file_path, new_file_path) = match file_paths.await {
                    Ok(file_paths) => file_paths,
                    Err(error) => {
                        if opts.continue_on_error {
                            println!("{} {}", "Error:".bright_red(), error);
                            return Ok(());
                        } else {
                            return Err(error);
                        }
                    }
                };

                targets
                    .write()
                    .expect(
                        format!(
                            "{} Poisoned sync-lock on write!",
                            "Fatal Error:".bright_red()
                        )
                        .as_str(),
                    )
                    .insert(new_file_path.clone());

                if opts.verbose >= 1 {
                    println!(
                        "{} {} {}",
                        old_file_path.to_string_lossy().red(),
                        "->".blue(),
                        new_file_path.to_string_lossy().green()
                    );
                }
                if opts.run {
                    fs::rename(old_file_path, new_file_path).await?;
                }
                Ok(())
            }
        })
        .await
}
