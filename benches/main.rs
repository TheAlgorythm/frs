extern crate criterion;

use frs::utils::SelectMapExt;

#[path = "../test-utils/select_map.rs"]
mod select_map_utils;

use criterion::{async_executor::AsyncStdExecutor, criterion_group, criterion_main, Criterion};

pub fn benchmark_select_map(c: &mut Criterion) {
    c.bench_function("SelectMap 99 Stream", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| select_map_utils::collected_99_stream())
    });
    c.bench_function("SelectMap 49 Stream", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| select_map_utils::collected_49_stream())
    });
    c.bench_function("SelectMap 999 Stream", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| select_map_utils::collected_999_stream())
    });
    c.bench_function("SelectMap Balanced 999 Stream", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| select_map_utils::collected_balanced_999_stream())
    });
    c.bench_function("SelectMap Unbalanced 999 Stream", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| select_map_utils::collected_unbalanced_999_stream())
    });
}

criterion_group!(benches, benchmark_select_map);
criterion_main!(benches);
