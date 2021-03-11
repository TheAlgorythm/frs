use super::cli;
use super::replace;
use crate::utils::SelectMapExt;
use async_std::sync::RwLock;
use async_std::{fs, io, path::PathBuf};
use colored::Colorize;
use futures::stream::{Stream, StreamExt, TryStreamExt};
use std::collections::HashSet;
use std::rc::Rc;

#[cfg(test)]
#[path = "./fs_test.rs"]
mod fs_test;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Replace(#[from] replace::Error),
    #[error("The parent directory `{}` does not exist", .0.to_string_lossy())]
    NonExistingParent(PathBuf),
}

pub async fn rename(opts: &cli::Cli, replacer: &replace::Replacer) -> Result<(), Error> {
    let done_targets = Rc::new(RwLock::new(HashSet::new()));
    read_dir(&opts)
        .await?
        .filter_map(async move |file_entry| check_file_type(file_entry, opts).await)
        .try_filter(|file_path| {
            let done_targets = Rc::clone(&done_targets);
            let file_path = file_path.clone();
            async move { check_unique_pattern_match(&file_path, &replacer, done_targets).await }
        })
        .map_ok(async move |file_path| rename_file_path(file_path, &replacer).await)
        .try_for_each_concurrent(None, |file_paths| {
            let done_targets = Rc::clone(&done_targets);
            async move { process_file_rename(file_paths.await, opts, done_targets).await }
        })
        .await
}

async fn read_dir(
    opts: &cli::Cli,
) -> Result<Box<dyn Stream<Item = io::Result<fs::DirEntry>> + Unpin>, io::Error> {
    if !opts.traverse_tree {
        Ok(Box::new(fs::read_dir(&opts.base_path).await?))
    } else {
        read_dir_recursive(&opts.base_path).await
    }
}

async fn read_dir_recursive(
    base_path: &PathBuf,
) -> Result<Box<dyn Stream<Item = io::Result<fs::DirEntry>> + Unpin>, io::Error> {
    Ok(Box::new(fs::read_dir(base_path).await?.select_map(
        |sub_path: &io::Result<fs::DirEntry>| {
            let sub_path = match sub_path {
                Ok(sub_path) => Some(sub_path.clone()),
                Err(_) => None,
            };
            Box::pin(async {
                let sub_path = sub_path?;
                if !sub_path.file_type().await.ok()?.is_dir() {
                    return None;
                }
                read_dir_recursive(&sub_path.path()).await.ok()
            })
        },
    )))
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
    replacer: &replace::Replacer,
    done_targets: Rc<RwLock<HashSet<PathBuf>>>,
) -> bool {
    !done_targets.read().await.contains(file_path) && replacer.is_match(file_path).unwrap_or(true)
}

async fn rename_file_path(
    file_path: PathBuf,
    replacer: &replace::Replacer,
) -> Result<(PathBuf, PathBuf), Error> {
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
