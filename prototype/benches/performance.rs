use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn my_function_to_benchmark() {
    // Simulate some heavy computation
    let mut sum = 0;
    for i in 0..1000 {
        sum += i;
    }
    black_box(sum); // Prevent compiler optimizations that could distort results
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("my_function_to_benchmark", |b| b.iter(|| my_function_to_benchmark()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
