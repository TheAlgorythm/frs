use crate::cli::cli_test::empty_cli;
use crate::stats::Stats;
use super::*;
use crate::replace::replace_test::{empty_replacer, restrictive_replacer};

#[async_std::test]
async fn check_io_error() {
    let cli = empty_cli();

    assert_matches!(
        check_file_type(Err(io::Error::new(io::ErrorKind::Other, "test")), &cli).await,
        Some(Err(Error::Io(_)))
    );
}

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
async fn simple_rename() {
    let old_path = PathBuf::from("./_old");
    let new_path = PathBuf::from("./old");

    assert_eq!(
        rename_file_path(old_path.clone(), &restrictive_replacer())
            .await
            .unwrap(),
        (old_path, new_path)
    );
}

#[async_std::test]
async fn stop_handle_error_on_ok() {
    let cli = empty_cli();
    let files_result = Ok(5);

    assert_matches!(handle_error_to_user(files_result, &cli, &Stats::new()).await, Some(Ok(_)));
}

#[async_std::test]
async fn continue_handle_error_on_ok() {
    let mut cli = empty_cli();
    cli.continue_on_error = true;
    let files_result = Ok(5);

    assert_matches!(handle_error_to_user(files_result, &cli, &Stats::new()).await, Some(Ok(_)));
}

#[async_std::test]
async fn stop_handle_error_on_error() {
    let cli = empty_cli();
    let files_result: Result<(), Error> = Err(Error::NonExistingParent(PathBuf::from("./old")));

    assert_matches!(
        handle_error_to_user(files_result, &cli, &Stats::new()).await,
        Some(Err(Error::NonExistingParent(_)))
    );
}

#[async_std::test]
async fn continue_handle_error_on_error() {
    let mut cli = empty_cli();
    cli.continue_on_error = true;
    let files_result: Result<(), Error> = Err(Error::NonExistingParent(PathBuf::from("./old")));

    assert_matches!(handle_error_to_user(files_result, &cli, &Stats::new()).await, None);
}

#[async_std::test]
async fn stop_on_error() {
    let done_targets = Rc::new(RwLock::new(hashset![]));
    let cli = empty_cli();
    let files_result = Err(Error::NonExistingParent(PathBuf::from("./old")));

    assert_matches!(
        process_file_rename(files_result, &cli, done_targets, &Stats::new()).await,
        Err(Error::NonExistingParent(_))
    );
}

#[async_std::test]
async fn continue_on_error() {
    let done_targets = Rc::new(RwLock::new(hashset![]));
    let mut cli = empty_cli();
    cli.continue_on_error = true;
    let files_result = Err(Error::NonExistingParent(PathBuf::from("./old")));

    assert_matches!(
        process_file_rename(files_result, &cli, done_targets, &Stats::new()).await,
        Ok(())
    );
}

#[async_std::test]
async fn add_to_done() {
    let done_targets = Rc::new(RwLock::new(hashset![]));
    let cli = empty_cli();
    let new_path = PathBuf::from("./new");
    let files_result = Ok((PathBuf::from("./old"), new_path.clone()));

    {
        let done_targets = done_targets.clone();
        assert_matches!(
            process_file_rename(files_result, &cli, done_targets, &Stats::new()).await,
            Ok(())
        );
    }
    assert!(done_targets.read().await.contains(&new_path));
}
