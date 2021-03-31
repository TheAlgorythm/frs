extern crate criterion;

use criterion::{async_executor::AsyncStdExecutor, criterion_group, criterion_main, Criterion};

use async_std::task::sleep;
use frs::utils::select_map::SelectMapExt;
use futures::stream;
use futures::stream::StreamExt;
use std::time::Duration;

async fn collected_99_stream() -> Vec<u16> {
    stream::iter(1_u16..=9_u16)
        .select_map(|primary_num| {
            let primary_num = primary_num.clone();
            Box::pin(async move {
                sleep(Duration::from_millis(21)).await;
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
                sleep(Duration::from_millis(21)).await;
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
                                sleep(Duration::from_millis(21)).await;
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

pub fn benchmark_select_map(c: &mut Criterion) {
    c.bench_function("SelectMap 99 Stream", |b| {
        b.to_async(AsyncStdExecutor).iter(|| collected_99_stream())
    });
    c.bench_function("SelectMap 49 Stream", |b| {
        b.to_async(AsyncStdExecutor).iter(|| collected_49_stream())
    });
    c.bench_function("SelectMap 999 Stream", |b| {
        b.to_async(AsyncStdExecutor).iter(|| collected_999_stream())
    });
    c.bench_function("SelectMap Balanced 999 Stream", |b| {
        b.to_async(AsyncStdExecutor).iter(|| collected_balanced_999_stream())
    });
    c.bench_function("SelectMap Unbalanced 999 Stream", |b| {
        b.to_async(AsyncStdExecutor).iter(|| collected_unbalanced_999_stream())
    });
}

criterion_group!(benches, benchmark_select_map);
criterion_main!(benches);
