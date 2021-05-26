use super::cli;
use super::replace;
use super::stats::Stats;
use crate::utils::SelectMapExt;
use async_std::sync::RwLock;
use async_std::{fs, io, path::PathBuf, stream};
use bool_ext::BoolExt;
use futures::stream::{Stream, StreamExt, TryStreamExt};
use std::collections::HashSet;
use std::rc::Rc;

#[cfg(test)]
#[path = "./fs_test.rs"]
mod fs_test;

#[cfg(test)]
pub use fs_test::FileInfo;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Replace(#[from] replace::Error),
    #[error("The parent directory `{}` does not exist", .0.to_string_lossy())]
    NonExistingParent(PathBuf),
}

pub async fn rename(
    opts: &cli::Cli,
    replacer: &replace::Replacer,
    stats: &Stats,
) -> Result<(), Error> {
    let done_targets = Rc::new(RwLock::new(HashSet::new()));
    read_dir(&opts)
        .await?
        .filter_map(|file_entry| async { check_file_type(file_entry, opts).await })
        .try_filter(|file| {
            let done_targets = Rc::clone(&done_targets);
            let file = file.clone();
            async move { check_unique_pattern_match(&file, &replacer, done_targets).await }
        })
        .map_ok(|file| async { rename_file_path(file, &replacer).await })
        .filter_map(|rename_info| async { handle_error_to_user(rename_info, opts, &stats).await })
        .try_for_each_concurrent(None, |rename_info| {
            let done_targets = Rc::clone(&done_targets);
            async { process_file_rename(rename_info.await, opts, done_targets, &stats).await }
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
        // traverse directory tree
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
                Some(
                    read_dir_recursive(&sub_path.path())
                        .await
                        .unwrap_or_else(|error| Box::new(stream::once(Err(error)))),
                )
            })
        },
    )))
}

#[cfg(not(test))]
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub file_type: fs::FileType,
}

#[cfg(not(test))]
impl FileInfo {
    fn new(path: PathBuf, file_type: fs::FileType) -> Self {
        Self { path, file_type }
    }
}

async fn check_file_type(
    file_entry: io::Result<fs::DirEntry>,
    opts: &cli::Cli,
) -> Option<Result<FileInfo, Error>> {
    let file_entry = try_wrap_err!(file_entry);
    let file_type = try_wrap_err!(file_entry.file_type().await);

    ((file_type.is_file() && opts.file)
        || (file_type.is_dir() && opts.directory)
        || (file_type.is_symlink() && opts.symlink))
        .some_with(|| Ok(FileInfo::new(file_entry.path(), file_type)))
}

async fn check_unique_pattern_match(
    file: &FileInfo,
    replacer: &replace::Replacer,
    done_targets: Rc<RwLock<HashSet<PathBuf>>>,
) -> bool {
    !done_targets.read().await.contains(&file.path) && replacer.is_match(&file.path).unwrap_or(true)
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RenameInfo {
    pub old_file: FileInfo,
    pub new_path: PathBuf,
}

async fn rename_file_path(
    old_file: FileInfo,
    replacer: &replace::Replacer,
) -> Result<RenameInfo, Error> {
    let new_path = replacer.replace(&old_file.path)?;
    new_path
        .parent()
        .expect("Couldn't get parent!")
        .is_dir()
        .await
        .err_with(|| {
            Error::NonExistingParent(
                new_path
                    .parent()
                    .expect("Couldn't get parent!")
                    .to_path_buf(),
            )
        })?;

    Ok(RenameInfo { old_file, new_path })
}

async fn handle_error_to_user<T>(
    file_paths: Result<T, Error>,
    opts: &cli::Cli,
    stats: &Stats,
) -> Option<Result<T, Error>> {
    match (file_paths, opts.continue_on_error) {
        (Ok(file_paths), _) => Some(Ok(file_paths)),
        (Err(error), false) => Some(Err(error)),
        (Err(error), true) => {
            stats.error(&error);
            None
        }
    }
}

async fn process_file_rename(
    rename_info: Result<RenameInfo, Error>,
    opts: &cli::Cli,
    done_targets: Rc<RwLock<HashSet<PathBuf>>>,
    stats: &Stats,
) -> Result<(), Error> {
    let rename_info = match (rename_info, opts.continue_on_error) {
        (Ok(rename_info), _) => rename_info,
        (Err(error), false) => return Err(error),
        (Err(error), true) => {
            stats.error(&error);
            return Ok(());
        }
    };

    done_targets
        .write()
        .await
        .insert(rename_info.new_path.clone());

    stats.rename(&rename_info);

    if opts.run {
        if let Err(error) = fs::rename(rename_info.old_file.path, rename_info.new_path).await {
            if opts.continue_on_error {
                stats.error(&error);
                return Ok(());
            } else {
                return Err(error.into());
            }
        }
    }
    Ok(())
}
