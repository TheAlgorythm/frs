use super::*;
use crate::cli::cli_test::empty_cli;
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
