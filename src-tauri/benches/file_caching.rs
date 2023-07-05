use std::path::Path;
use mielikki::filecache;
use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(feature = "benchmarking")]
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("size-10");
    group.sample_size(10);
    let fc = filecache::FileCache::new(Path::new("cache/bench-cache").to_owned()).unwrap();
    println!("Running!");
    group.bench_function("Update memory cache", |b|  b.iter(|| fc.update_memory_cache()));
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);