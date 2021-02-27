use super::cli;
use super::replace;
use async_std::sync::{Arc, RwLock};
use async_std::{fs, io, path::PathBuf, task};
use colored::*;
use futures::stream::{StreamExt, TryStreamExt};
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

#[cfg(test)]
#[path = "./fs_test.rs"]
mod fs_test;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Replace(replace::Error),
    NonExistingParent(PathBuf),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<replace::Error> for Error {
    fn from(err: replace::Error) -> Self {
        Self::Replace(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Io(ref err) => err.fmt(f),
            Self::Replace(ref err) => err.fmt(f),
            Self::NonExistingParent(ref parent) => write!(
                f,
                "The parent directory `{}` does not exist",
                parent.to_string_lossy()
            ),
        }
    }
}

pub async fn rename(opts: &cli::Cli, replacer: Arc<replace::Replacer>) -> Result<(), Error> {
    let done_targets = Rc::new(RwLock::new(HashSet::new()));
    fs::read_dir(opts.base_path.clone())
        .await?
        .filter_map(async move |file_entry| check_file_type(file_entry, opts).await)
        .try_filter(|file_path| {
            let file_path = file_path.clone();
            let done_targets = Rc::clone(&done_targets);
            let replacer = Arc::clone(&replacer);
            async move { check_unique_pattern_match(&file_path, replacer, done_targets).await }
        })
        .map_ok(|file_path| {
            let replacer = Arc::clone(&replacer);
            async move { rename_file_path(file_path, replacer).await }
        })
        .try_for_each_concurrent(None, |file_paths| {
            let done_targets = Rc::clone(&done_targets);
            async move { process_file_rename(file_paths.await, opts, done_targets).await }
        })
        .await
}

async fn check_file_type(
    file_path: io::Result<fs::DirEntry>,
    opts: &cli::Cli,
) -> Option<Result<PathBuf, Error>> {
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
}

async fn check_unique_pattern_match(
    file_path: &PathBuf,
    replacer: Arc<replace::Replacer>,
    done_targets: Rc<RwLock<HashSet<PathBuf>>>,
) -> bool {
    let file_path = file_path.clone();
    !done_targets.read().await.contains(&file_path)
        && task::spawn(async move { replacer.is_match(&file_path).unwrap_or(true) }).await
}

async fn rename_file_path(
    file_path: PathBuf,
    replacer: Arc<replace::Replacer>,
) -> Result<(PathBuf, PathBuf), Error> {
    let new_file_path = {
        let file_path = file_path.clone();
        task::spawn(async move { replacer.replace(&file_path) }).await?
    };
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
}

async fn process_file_rename(
    file_paths: Result<(PathBuf, PathBuf), Error>,
    opts: &cli::Cli,
    done_targets: Rc<RwLock<HashSet<PathBuf>>>,
) -> Result<(), Error> {
    let (old_file_path, new_file_path) = match file_paths {
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

    done_targets.write().await.insert(new_file_path.clone());

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
