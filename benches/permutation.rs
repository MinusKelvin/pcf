use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use pcf::{ BitBoard, SearchStatus };
use pcf::Piece::*;

fn benchmark(c: &mut Criterion) {
    c.bench_function("2 line perm", |b| b.iter(||
        pcf::solve_pc_at_height(
            black_box(&[I, L, S, J, O, O]),
            black_box(BitBoard::filled(0)),
            true, false, 4, pcf::placeability::always,
            |_| SearchStatus::Continue
        )
    ));
    c.bench_function("PCO perm", |b| b.iter(||
        pcf::solve_pc_at_height(
            black_box(&[I, T, O, J]),
            black_box(BitBoard(0b1111000011_1111000111_1111001111_1111000111)),
            true, false, 4, pcf::placeability::always,
            |_| SearchStatus::Continue
        )
    ));
    c.bench_function("grace system perm", |b| b.iter(||
        pcf::solve_pc_at_height(
            black_box(&[T, I, T, O, J]),
            black_box(BitBoard(0b1111110000_1111110000_1111110000_1111110000)),
            true, false, 4, pcf::placeability::always,
            |_| SearchStatus::Continue
        )
    ));
    c.bench_function("ISZL 100% perm", |b| b.iter(||
        pcf::solve_pc_at_height(
            black_box(&[J, T, O, L, S, Z, T]),
            black_box(BitBoard(0b0000001111_0000000111_0000011111_0000001111)),
            true, false, 4, pcf::placeability::always,
            |_| SearchStatus::Continue
        )
    ));
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(20));
    targets = benchmark
}
criterion_main!(benches);