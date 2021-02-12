use colored::*;
use futures::stream::{StreamExt, TryStreamExt};
use async_std::{future, fs, io};
use std::collections::HashSet;
use std::sync::RwLock;
use std::rc::Rc;

pub async fn rename(opts: &super::cli::Cli, replacer: &super::replace::Replacer) -> Result<(), io::Error> {
    let targets = Rc::new(RwLock::new(HashSet::new()));
    fs::read_dir(opts.base_path.clone())
        .await?
        .filter_map(async move |file| {
            let file = match file {
                Ok(file) => file,
                Err(error) => return Some(Err(error)),
            };
            let file_type = file.file_type().await.unwrap();
            if (file_type.is_file() && opts.file) ||
                (file_type.is_dir() && opts.directory) ||
                (file_type.is_symlink() && opts.symlink) {
                return Some(Ok(file.path()));
            }
            None
        })
        .try_filter(|file| future::ready(!targets.read().unwrap().contains(file) && replacer.is_match(file).unwrap_or(true)))
        .map_ok(async move |file| {
            Ok((file.clone(), replacer.replace(&file).unwrap()))
        })
        .try_for_each(|file_futures| {
            let targets = targets.clone();
            async move {
                let (old_file, new_file) = match file_futures.await {
                    Ok(files) => files,
                    Err(error) => if opts.continue_on_error {
                        println!("{} {}", "Error:".bright_red(), error);
                        return Ok(());
                    } else {
                        return Err(error);
                    }
                };
                
                targets.write().unwrap().insert(new_file.clone());

                if opts.verbose >= 1 {
                    println!("{} {} {}", old_file.to_str().unwrap().red(), "->".blue(), new_file.to_str().unwrap().green());
                }
                if opts.run {
                    fs::rename(old_file, new_file).await.unwrap();
                }
                Ok(())
            }
        })
        .await
    
}
