use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::RngCore;

// todo: add benchmarks for other encoding functions.
fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("to-string");

    for size in [2, 8, 32, 128, 512, 2048] {
        let mut input = vec![0; size];

        rand::thread_rng().fill_bytes(&mut input);

        group
            .bench_with_input(BenchmarkId::new("hex", input.len()), &input, |b, input| {
                b.iter(|| hex::encode(&input))
            })
            .bench_with_input(
                BenchmarkId::new("faster-hex", input.len()),
                &input,
                |b, input| b.iter(|| faster_hex::hex_string(&input)),
            )
            .bench_with_input(
                BenchmarkId::new("hector", input.len()),
                &input,
                |b, input| b.iter(|| hector::encode(&input)),
            );
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
