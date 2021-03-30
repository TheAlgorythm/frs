#[macro_use]
extern crate criterion;

use criterion::{async_executor::AsyncStdExecutor, criterion_group, criterion_main, Criterion};

use frs::utils::select_map::select_map_test;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.to_async(AsyncStdExecutor)
            .iter(|| select_map_test::collected_999_stream())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
