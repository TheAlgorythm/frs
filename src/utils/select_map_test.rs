use super::*;

#[path = "../../test-utils/select_map.rs"]
mod select_map_utils;

#[async_std::test]
async fn stream_1_to_9() {
    let mut streamed = select_map_utils::collected_eliminated_single_recursive_stream(10).await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=9).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_99() {
    let mut streamed = select_map_utils::collected_single_recursive_stream(10).await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=99).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_49() {
    let mut streamed = select_map_utils::collected_filtered_single_recursive_stream(10).await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=59).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_999() {
    let mut streamed = select_map_utils::collected_double_recursive_stream(10).await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=999).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_999_balanced() {
    let mut streamed = select_map_utils::collected_balanced_double_recursive_stream(10).await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=999).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_999_unbalanced() {
    let mut streamed = select_map_utils::collected_unbalanced_double_recursive_stream(10).await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=999).collect::<Vec<_>>());
}
