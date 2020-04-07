use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use pcf::{ BitBoard, PieceSet };
use pcf::Piece::*;

fn benchmark(c: &mut Criterion) {
    c.bench_function("2 line com", |b| b.iter(||
        pcf::combination::find_combinations(
            black_box(PieceSet::default()
                .with(I).with(I).with(I).with(I).with(I)
                .with(L).with(L).with(L).with(L).with(L)
                .with(J).with(J).with(J).with(J).with(J)
                .with(O).with(O).with(O).with(O).with(O)
            ), black_box(BitBoard::filled(0)), 2
        )
    ));
    c.bench_function("PCO com", |b| b.iter(||
        pcf::combination::find_combinations(
            black_box(PieceSet::default()
                .with(I)
                .with(L)
                .with(J)
                .with(O)
                .with(S)
                .with(Z)
                .with(T)
            ), black_box(BitBoard(0b1111000011_1111000111_1111001111_1111000111)), 4
        )
    ));
    c.bench_function("grace system com", |b| b.iter(||
        pcf::combination::find_combinations(
            black_box(PieceSet::default()
                .with(I)
                .with(L)
                .with(J)
                .with(O)
                .with(S)
                .with(Z)
                .with(T).with(T)
            ), black_box(BitBoard(0b1111110000_1111110000_1111110000_1111110000)), 4
        )
    ));
    c.bench_function("ISZL 100% com", |b| b.iter(||
        pcf::combination::find_combinations(
            black_box(PieceSet::default()
                .with(I)
                .with(L)
                .with(J).with(J)
                .with(O).with(O)
                .with(S)
                .with(Z)
                .with(T).with(T)
            ), black_box(BitBoard(0b0000001111_0000000111_0000011111_0000001111)), 4
        )
    ));
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(20));
    targets = benchmark
}
criterion_main!(benches);