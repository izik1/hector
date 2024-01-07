use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use rand::RngCore;

// todo: add benchmarks for other encoding functions.
fn bench(c: &mut Criterion) {
    const SIZES: [usize; 6] = [2, 8, 32, 128, 512, 2048];
    {
        let mut group = c.benchmark_group("to-string");

        for size in SIZES {
            let mut input = vec![0; size];

            rand::thread_rng().fill_bytes(&mut input);

            group
                .throughput(Throughput::Bytes(input.len() as u64))
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

    let mut group = c.benchmark_group("to-slice");

    for size in SIZES {
        let mut input = vec![0; size];

        rand::thread_rng().fill_bytes(&mut input);

        group
            .throughput(Throughput::Bytes(input.len() as u64))
            .bench_with_input(BenchmarkId::new("hex", input.len()), &input, |b, input| {
                b.iter_batched(
                    || (input, vec![0; size * 2]),
                    |(input, mut output)| {
                        hex::encode_to_slice(&input, &mut output).unwrap();
                        unsafe { String::from_utf8_unchecked(output) }
                    },
                    BatchSize::SmallInput,
                )
            })
            .bench_with_input(
                BenchmarkId::new("faster-hex", input.len()),
                &input,
                |b, input| {
                    b.iter_batched(
                        || (input, vec![0; size * 2]),
                        |(input, mut output)| {
                            faster_hex::hex_encode(&input, &mut output).unwrap();
                            unsafe { String::from_utf8_unchecked(output) }
                        },
                        BatchSize::SmallInput,
                    )
                },
            )
            .bench_with_input(
                BenchmarkId::new("hector", input.len()),
                &input,
                |b, input| {
                    b.iter_batched(
                        || (input, vec![0; size * 2]),
                        |(input, mut output)| {
                            hector::encode_to_slice(&input, &mut output).unwrap();
                            unsafe { String::from_utf8_unchecked(output) }
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
