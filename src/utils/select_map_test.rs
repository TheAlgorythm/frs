use super::*;
use async_std::task::sleep;
use futures::stream;
use std::time::Duration;

async fn collected_99_stream() -> Vec<u16> {
    stream::iter(1_u16..=9_u16)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(42)).await;
                Some(Box::new(stream::iter(
                    10 * primary_num..=10 * primary_num + 9,
                )))
            })
        })
        .collect()
        .await
}

async fn collected_49_stream() -> Vec<u16> {
    stream::iter(1_u16..=9_u16)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(42)).await;
                if primary_num > 4 {
                    return None;
                }
                Some(Box::new(stream::iter(
                    10 * primary_num..=10 * primary_num + 9,
                )))
            })
        })
        .collect()
        .await
}

async fn collected_999_stream() -> Vec<u16> {
    stream::iter(1_u16..=9_u16)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(42)).await;
                Some(Box::new(
                    stream::iter(10 * primary_num..=10 * primary_num + 9).select_map(
                        |secondary_num| {
                            let secondary_num = secondary_num.clone();
                            Box::pin(async move {
                                sleep(Duration::from_millis(42)).await;
                                Some(Box::new(stream::iter(
                                    10 * secondary_num..=10 * secondary_num + 9,
                                )))
                            })
                        },
                    ),
                ))
            })
        })
        .collect()
        .await
}

async fn collected_balanced_999_stream() -> Vec<u16> {
    stream::iter(1_u16..=9_u16)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(42 - primary_num as u64)).await;
                Some(Box::new(
                    stream::iter(10 * primary_num..=10 * primary_num + 9).select_map(
                        move |secondary_num| {
                            let secondary_num = secondary_num.clone();
                            Box::pin(async move {
                                sleep(Duration::from_millis(21 + primary_num as u64)).await;
                                Some(Box::new(stream::iter(
                                    10 * secondary_num..=10 * secondary_num + 9,
                                )))
                            })
                        },
                    ),
                ))
            })
        })
        .collect()
        .await
}

async fn collected_unbalanced_999_stream() -> Vec<u16> {
    stream::iter(1_u16..=9_u16)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(42 - primary_num as u64)).await;
                Some(Box::new(
                    stream::iter(10 * primary_num..=10 * primary_num + 9).select_map(
                        move |secondary_num| {
                            let secondary_num = secondary_num.clone();
                            Box::pin(async move {
                                sleep(Duration::from_millis(21 - primary_num as u64)).await;
                                Some(Box::new(stream::iter(
                                    10 * secondary_num..=10 * secondary_num + 9,
                                )))
                            })
                        },
                    ),
                ))
            })
        })
        .collect()
        .await
}

#[async_std::test]
async fn stream_1_to_99() {
    let mut streamed = collected_99_stream().await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=99).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_49() {
    let mut streamed = collected_49_stream().await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=49).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_999() {
    let mut streamed = collected_999_stream().await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=999).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_999_balanced() {
    let mut streamed = collected_balanced_999_stream().await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=999).collect::<Vec<_>>());
}

#[async_std::test]
async fn stream_1_to_999_unbalanced() {
    let mut streamed = collected_unbalanced_999_stream().await;
    streamed.sort_unstable();
    assert_eq!(streamed, (1..=999).collect::<Vec<_>>());
}
