use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use pcf::{ BitBoard, Placement, PieceState };
use pcf::Piece::*;
use std::sync::atomic::{ AtomicBool, Ordering };

fn benchmark(c: &mut Criterion) {
    c.bench_function("solve combo harddrop only", |b| b.iter(||
        pcf::solve_placement_combination(
            black_box(&[J, L, O, I, T, S, Z, J, L, I, O]),
            black_box(BitBoard(0)),
            &[
                Placement { kind: PieceState::IVertical0000, x: 0 },
                Placement { kind: PieceState::O00, x: 1 },
                Placement { kind: PieceState::O20, x: 1 },
                Placement { kind: PieceState::TEast000, x: 3 },
                Placement { kind: PieceState::LWest010, x: 3 },
                Placement { kind: PieceState::JNorth00, x: 5 },
                Placement { kind: PieceState::SVertical100, x: 5 },
                Placement { kind: PieceState::IHorizontal3, x: 6 },
                Placement { kind: PieceState::ZHorizontal00, x: 7 },
                Placement { kind: PieceState::JSouth10, x: 7 },
            ],
            true, false, &AtomicBool::new(false),
            pcf::placeability::hard_drop_only,
            |_| {}
        )
    ));
    c.bench_function("solve combo always placeable", |b| b.iter(||
        pcf::solve_placement_combination(
            black_box(&[J, L, O, I, T, S, Z, J, L, I, O]),
            black_box(BitBoard(0)),
            &[
                Placement { kind: PieceState::IVertical0000, x: 0 },
                Placement { kind: PieceState::O00, x: 1 },
                Placement { kind: PieceState::O20, x: 1 },
                Placement { kind: PieceState::TEast000, x: 3 },
                Placement { kind: PieceState::LWest010, x: 3 },
                Placement { kind: PieceState::JNorth00, x: 5 },
                Placement { kind: PieceState::SVertical100, x: 5 },
                Placement { kind: PieceState::IHorizontal3, x: 6 },
                Placement { kind: PieceState::ZHorizontal00, x: 7 },
                Placement { kind: PieceState::JSouth10, x: 7 },
            ],
            true, false, &AtomicBool::new(false),
            pcf::placeability::always,
            |_| {}
        )
    ));
    c.bench_function("solve combo tucks", |b| b.iter(||
        pcf::solve_placement_combination(
            black_box(&[S, Z, O, J, T, L, I, Z, L, J, T]),
            black_box(BitBoard(0)),
            &[
                Placement { kind: PieceState::JWest000, x: 0 },
                Placement { kind: PieceState::JEast100, x: 0 },
                Placement { kind: PieceState::O00, x: 2 },
                Placement { kind: PieceState::LSouth20, x: 2 },
                Placement { kind: PieceState::TSouth10, x: 3 },
                Placement { kind: PieceState::SHorizontal00, x: 4 },
                Placement { kind: PieceState::ZHorizontal20, x: 5 },
                Placement { kind: PieceState::TNorth00, x: 6 },
                Placement { kind: PieceState::LWest100, x: 7 },
                Placement { kind: PieceState::IVertical0000, x: 9 },
            ],
            true, false, &AtomicBool::new(false),
            pcf::placeability::tucks,
            |_| {}
        )
    ));
    c.bench_function("solve combo simple srs", |b| b.iter(||
        pcf::solve_placement_combination(
            black_box(&[S, Z, I, L, T, J, O, T, J, O, Z]),
            black_box(BitBoard(0)),
            &[
                Placement { kind: PieceState::O00, x: 0 },
                Placement { kind: PieceState::O20, x: 0 },
                Placement { kind: PieceState::TNorth00, x: 2 },
                Placement { kind: PieceState::JEast100, x: 2 },
                Placement { kind: PieceState::SHorizontal20, x: 3 },
                Placement { kind: PieceState::ZHorizontal00, x: 4 },
                Placement { kind: PieceState::ZHorizontal10, x: 5 },
                Placement { kind: PieceState::IHorizontal3, x: 6 },
                Placement { kind: PieceState::LNorth00, x: 7 },
                Placement { kind: PieceState::TSouth10, x: 7 },
            ],
            true, false, &AtomicBool::new(false),
            pcf::placeability::simple_srs_spins,
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