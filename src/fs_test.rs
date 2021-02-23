use super::*;
use crate::replace::replace_test::{empty_replacer, restrictive_replacer};

#[async_std::test]
async fn already_done_matching_target() {
    let done_path = PathBuf::from("/done");
    let done_targets = Rc::new(RwLock::new(hashset![
        done_path.clone(),
        PathBuf::from("done-2")
    ]));

    assert!(!check_unique_pattern_match(&done_path, &empty_replacer(), done_targets).await);
}

#[async_std::test]
async fn not_done_matching_target() {
    let done_targets = Rc::new(RwLock::new(hashset![
        PathBuf::from("/done"),
        PathBuf::from("done-2")
    ]));

    assert!(
        check_unique_pattern_match(&PathBuf::from("/new"), &empty_replacer(), done_targets).await
    );
}

#[async_std::test]
async fn already_done_not_matching_target() {
    let done_path = PathBuf::from("/done");
    let done_targets = Rc::new(RwLock::new(hashset![
        done_path.clone(),
        PathBuf::from("done-2")
    ]));

    assert!(!check_unique_pattern_match(&done_path, &restrictive_replacer(), done_targets).await);
}

#[async_std::test]
async fn not_done_not_matching_target() {
    let done_targets = Rc::new(RwLock::new(hashset![
        PathBuf::from("/done"),
        PathBuf::from("done-2")
    ]));

    assert!(
        !check_unique_pattern_match(
            &PathBuf::from("/new"),
            &restrictive_replacer(),
            done_targets
        )
        .await
    );
}

#[async_std::test]
async fn pass_matching_error() {
    let done_targets = Rc::new(RwLock::new(hashset![
        PathBuf::from("/done"),
        PathBuf::from("done-2")
    ]));

    assert!(
        check_unique_pattern_match(&PathBuf::from(".."), &restrictive_replacer(), done_targets)
            .await
    );
}

#[async_std::test]
async fn rename_invalid_filename() {
    assert_matches!(
        rename_file_path(PathBuf::from("."), &restrictive_replacer()).await,
        Err(Error::Replace(replace::Error::InvalidFileName(_)))
    );
}

#[async_std::test]
async fn rename_without_parent() {
    assert_matches!(
        rename_file_path(PathBuf::from("non_existant/_old"), &restrictive_replacer()).await,
        Err(Error::NonExistingParent(_))
    );
}

#[async_std::test]
#[allow(unused_variables)]
async fn simple_rename() {
    let old_path = PathBuf::from("./_old");
    let new_path = PathBuf::from("./old");
    assert_matches!(
        rename_file_path(old_path, &restrictive_replacer()).await,
        Ok((old_path, new_path))
    );
}
