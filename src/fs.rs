use colored::*;
use futures::stream::StreamExt;
use async_std::{future, fs};
use std::collections::HashSet;
use std::sync::RwLock;
use std::rc::Rc;

pub async fn rename(opts: &super::cli::Cli, replacer: &super::replace::Replacer) {
    let targets = Rc::new(RwLock::new(HashSet::new()));
    fs::read_dir(opts.base_path.clone())
        .await
        .unwrap()
        .filter_map(async move |file| {
            let file_type = file.as_ref().unwrap().file_type().await.unwrap();
            if (file_type.is_file() && opts.file) ||
                (file_type.is_dir() && opts.directory) ||
                (file_type.is_symlink() && opts.symlink) {
                return Some(file);
            }
            None
        })
        .map(|file| file.unwrap().path())
        .filter(|file| future::ready(!targets.read().unwrap().contains(file) && replacer.is_match(file).unwrap_or(false)))
        .map(|file| future::ready((file.clone(), replacer.replace(&file).unwrap())))
        .for_each(|file_futures| {
            let targets = targets.clone();
            async move {
                let (old_file, new_file) = file_futures.await;
                
                targets.write().unwrap().insert(new_file.clone());

                if opts.verbose >= 1 {
                    println!("{} {} {}", old_file.to_str().unwrap().red(), "->".blue(), new_file.to_str().unwrap().green());
                }
                if opts.run {
                    fs::rename(old_file, new_file).await.unwrap();
                }
            }
        })
        .await;
    
}
