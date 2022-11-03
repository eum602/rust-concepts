// Benchmark that measures the speed of sploosh(8, 9, 10)
// - Speed up the implementation of sploosh(8, 9, 10) without breaking the other tests.
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use testing::sploosh;

pub fn bench_sploosh(c: &mut Criterion) {
    c.bench_function("my benchmark name for sploosh test 1", |b| {
        b.iter(|| sploosh(black_box(8), black_box(9), black_box(10)))
    }); // || sploosh(1) --> a closure
        // black_box to prevent compiler optimizations
}

criterion_group!(benches, bench_sploosh);
criterion_main!(benches);

// reports on target/criterion/report/index.html
