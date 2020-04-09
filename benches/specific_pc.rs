use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use pcf::{ BitBoard, SearchStatus };
use pcf::Piece::*;

fn benchmark(c: &mut Criterion) {
    c.bench_function("first PC", |b| b.iter(||
        pcf::solve_pc(
            black_box(&[L, T, I, J, Z, S, O, I, Z, O, J]),
            black_box(BitBoard::filled(0)),
            true, false, pcf::placeability::hard_drop_only,
            |_| SearchStatus::Abort
        )
    ));
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(20));
    targets = benchmark
}
criterion_main!(benches);