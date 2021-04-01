use super::SelectMapExt;
use async_std::task::sleep;
use futures::stream;
use futures::stream::StreamExt;
use std::time::Duration;

pub async fn collected_single_recursive_stream(base: u32) -> Vec<u32> {
    stream::iter(1..base)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(21)).await;
                Some(Box::new(stream::iter(
                    base * primary_num..base * primary_num + base,
                )))
            })
        })
        .collect()
        .await
}

pub async fn collected_eliminated_single_recursive_stream(base: u32) -> Vec<u32> {
    stream::iter(1..base)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(21)).await;
                if primary_num == base {
                    return Some(Box::new(stream::iter(
                        base * primary_num..base * primary_num + base,
                    )));
                }
                None
            })
        })
        .collect()
        .await
}

pub async fn collected_filtered_single_recursive_stream(base: u32) -> Vec<u32> {
    let maximal_filter = base / 2;
    stream::iter(1..base)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(21)).await;
                if primary_num > maximal_filter {
                    return None;
                }
                Some(Box::new(stream::iter(
                    base * primary_num..base * primary_num + base,
                )))
            })
        })
        .collect()
        .await
}

pub async fn collected_double_recursive_stream(base: u32) -> Vec<u32> {
    stream::iter(1..base)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(42)).await;
                Some(Box::new(
                    stream::iter(base * primary_num..base * primary_num + base).select_map(
                        move |secondary_num| {
                            let secondary_num = secondary_num.clone();
                            Box::pin(async move {
                                sleep(Duration::from_millis(21)).await;
                                Some(Box::new(stream::iter(
                                    base * secondary_num..base * secondary_num + base,
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

pub async fn collected_balanced_double_recursive_stream(base: u32) -> Vec<u32> {
    let primary_delay = base as u64 * 2;
    stream::iter(1..base)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(primary_delay - primary_num as u64)).await;
                Some(Box::new(
                    stream::iter(base * primary_num..base * primary_num + base).select_map(
                        move |secondary_num| {
                            let secondary_num = secondary_num.clone();
                            Box::pin(async move {
                                sleep(Duration::from_millis(base as u64 + primary_num as u64))
                                    .await;
                                Some(Box::new(stream::iter(
                                    base * secondary_num..base * secondary_num + base,
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

pub async fn collected_unbalanced_double_recursive_stream(base: u32) -> Vec<u32> {
    let primary_delay = base as u64 * 2;
    stream::iter(1..base)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(primary_delay - primary_num as u64)).await;
                Some(Box::new(
                    stream::iter(base * primary_num..base * primary_num + base).select_map(
                        move |secondary_num| {
                            let secondary_num = secondary_num.clone();
                            Box::pin(async move {
                                sleep(Duration::from_millis((base - primary_num) as u64)).await;
                                Some(Box::new(stream::iter(
                                    base * secondary_num..base * secondary_num + base,
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
