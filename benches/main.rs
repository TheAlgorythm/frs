extern crate criterion;

use frs::utils::SelectMapExt;

#[path = "../test-utils/select_map.rs"]
mod select_map_utils;

use criterion::{async_executor::AsyncStdExecutor, criterion_group, criterion_main, Criterion};

pub fn benchmark_select_map(c: &mut Criterion) {
    for base in vec![8, 32] {
        c.bench_function(
            format!("SelectMap {base} Stream", base = base).as_str(),
            |b| {
                b.to_async(AsyncStdExecutor)
                    .iter(|| select_map_utils::collected_eliminated_single_recursive_stream(base))
            },
        );
        c.bench_function(
            format!("SelectMap {base}x{base} Stream", base = base).as_str(),
            |b| {
                b.to_async(AsyncStdExecutor)
                    .iter(|| select_map_utils::collected_single_recursive_stream(base))
            },
        );
        c.bench_function(
            format!("SelectMap {base}/2 Stream", base = base).as_str(),
            |b| {
                b.to_async(AsyncStdExecutor)
                    .iter(|| select_map_utils::collected_filtered_single_recursive_stream(base))
            },
        );
        c.bench_function(
            format!(
                "SelectMap non sleeping {base}x{base}x{base} Stream",
                base = base
            )
            .as_str(),
            |b| {
                b.to_async(AsyncStdExecutor)
                    .iter(|| select_map_utils::collected_non_sleeping_double_recursive_stream(base))
            },
        );
        c.bench_function(
            format!("SelectMap Normal {base}x{base}x{base} Stream", base = base).as_str(),
            |b| {
                b.to_async(AsyncStdExecutor)
                    .iter(|| select_map_utils::collected_double_recursive_stream(base))
            },
        );
        c.bench_function(
            format!(
                "SelectMap Balanced {base}x{base}x{base} Stream",
                base = base
            )
            .as_str(),
            |b| {
                b.to_async(AsyncStdExecutor)
                    .iter(|| select_map_utils::collected_balanced_double_recursive_stream(base))
            },
        );
        c.bench_function(
            format!(
                "SelectMap Unbalanced {base}x{base}x{base} Stream",
                base = base
            )
            .as_str(),
            |b| {
                b.to_async(AsyncStdExecutor)
                    .iter(|| select_map_utils::collected_unbalanced_double_recursive_stream(base))
            },
        );
        c.bench_function(
            format!(
                "SelectMap non sleeping {base}x{base}x{base}x{base} Stream",
                base = base
            )
            .as_str(),
            |b| {
                b.to_async(AsyncStdExecutor)
                    .iter(|| select_map_utils::collected_non_sleeping_triple_recursive_stream(base))
            },
        );
        c.bench_function(
            format!(
                "SelectMap Normal {base}x{base}x{base}x{base} Stream",
                base = base
            )
            .as_str(),
            |b| {
                b.to_async(AsyncStdExecutor)
                    .iter(|| select_map_utils::collected_triple_recursive_stream(base))
            },
        );
    }
}

criterion_group!(benches, benchmark_select_map);
criterion_main!(benches);
