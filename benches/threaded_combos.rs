use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pcf::{BitBoard, Piece::*, PieceSet};
use std::sync::atomic::AtomicBool;

fn benchmark(c: &mut Criterion) {
    let mut threaded = c.benchmark_group("ISZL com");
    threaded.bench_function("1T", |b| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap()
            .install(|| {
                b.iter(|| {
                    pcf::find_combinations_mt(
                        black_box(
                            PieceSet::default()
                                .with(I)
                                .with(L)
                                .with(J)
                                .with(J)
                                .with(O)
                                .with(O)
                                .with(S)
                                .with(Z)
                                .with(T)
                                .with(T),
                        ),
                        black_box(BitBoard(0b0000001111_0000000111_0000011111_0000001111)),
                        &AtomicBool::new(false),
                        4,
                        |_| {},
                    )
                })
            })
    });
    threaded.bench_function("2T", |b| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap()
            .install(|| {
                b.iter(|| {
                    pcf::find_combinations_mt(
                        black_box(
                            PieceSet::default()
                                .with(I)
                                .with(L)
                                .with(J)
                                .with(J)
                                .with(O)
                                .with(O)
                                .with(S)
                                .with(Z)
                                .with(T)
                                .with(T),
                        ),
                        black_box(BitBoard(0b0000001111_0000000111_0000011111_0000001111)),
                        &AtomicBool::new(false),
                        4,
                        |_| {},
                    )
                })
            })
    });
    threaded.bench_function("4T", |b| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap()
            .install(|| {
                b.iter(|| {
                    pcf::find_combinations_mt(
                        black_box(
                            PieceSet::default()
                                .with(I)
                                .with(L)
                                .with(J)
                                .with(J)
                                .with(O)
                                .with(O)
                                .with(S)
                                .with(Z)
                                .with(T)
                                .with(T),
                        ),
                        black_box(BitBoard(0b0000001111_0000000111_0000011111_0000001111)),
                        &AtomicBool::new(false),
                        4,
                        |_| {},
                    )
                })
            })
    });
    threaded.bench_function("8T", |b| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap()
            .install(|| {
                b.iter(|| {
                    pcf::find_combinations_mt(
                        black_box(
                            PieceSet::default()
                                .with(I)
                                .with(L)
                                .with(J)
                                .with(J)
                                .with(O)
                                .with(O)
                                .with(S)
                                .with(Z)
                                .with(T)
                                .with(T),
                        ),
                        black_box(BitBoard(0b0000001111_0000000111_0000011111_0000001111)),
                        &AtomicBool::new(false),
                        4,
                        |_| {},
                    )
                })
            })
    });
    threaded.bench_function("12T", |b| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(12)
            .build()
            .unwrap()
            .install(|| {
                b.iter(|| {
                    pcf::find_combinations_mt(
                        black_box(
                            PieceSet::default()
                                .with(I)
                                .with(L)
                                .with(J)
                                .with(J)
                                .with(O)
                                .with(O)
                                .with(S)
                                .with(Z)
                                .with(T)
                                .with(T),
                        ),
                        black_box(BitBoard(0b0000001111_0000000111_0000011111_0000001111)),
                        &AtomicBool::new(false),
                        4,
                        |_| {},
                    )
                })
            })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(20));
    targets = benchmark
}
criterion_main!(benches);
