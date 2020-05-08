use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use pcf::{ BitBoard, Piece::* };
use std::sync::atomic::AtomicBool;

fn benchmark(c: &mut Criterion) {
    c.bench_function("2 line perm", |b| b.iter(||
        pcf::solve_pc(
            black_box(&[I, L, S, J, O, O]),
            black_box(BitBoard::filled(0)),
            true, false, &AtomicBool::new(false),
            pcf::placeability::always,
            |_| {}
        )
    ));
    c.bench_function("PCO perm", |b| b.iter(||
        pcf::solve_pc(
            black_box(&[I, T, O, J]),
            black_box(BitBoard(0b1111000011_1111000111_1111001111_1111000111)),
            true, false, &AtomicBool::new(false),
            pcf::placeability::always,
            |_| {}
        )
    ));
    c.bench_function("grace system perm", |b| b.iter(||
        pcf::solve_pc(
            black_box(&[T, I, T, O, J]),
            black_box(BitBoard(0b1111110000_1111110000_1111110000_1111110000)),
            true, false, &AtomicBool::new(false),
            pcf::placeability::always,
            |_| {}
        )
    ));
    c.bench_function("ISZL 100% perm", |b| b.iter(||
        pcf::solve_pc(
            black_box(&[J, T, O, L, S, Z, T]),
            black_box(BitBoard(0b0000001111_0000000111_0000011111_0000001111)),
            true, false, &AtomicBool::new(false),
            pcf::placeability::always,
            |_| {}
        )
    ));
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(20));
    targets = benchmark
}
criterion_main!(benches);