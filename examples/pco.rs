use fumen::{CellColor, Fumen};
use pcf::{BitBoard, Piece, PieceSet};
use rand::prelude::*;
use std::sync::atomic::AtomicBool;

mod common;

fn main() {
    let board = BitBoard(0b1100001111_1110001111_1111001111_1110001111);
    let pieces = PieceSet::default()
        .with(Piece::I)
        .with(Piece::T)
        .with(Piece::O)
        .with(Piece::S)
        .with(Piece::Z)
        .with(Piece::L)
        .with(Piece::J);

    let mut fumen = Fumen::default();
    common::blit(&mut fumen.pages[0], board, CellColor::Grey);

    pcf::find_combinations(pieces, board, &AtomicBool::new(false), 4, |combo| {
        let mut page = fumen.pages[0].clone();
        common::draw_placements(&mut page, &combo);
        fumen.pages.push(page);
    });

    println!(
        "Combinatorial PCO Solutions: http://fumen.zui.jp/?{}",
        fumen.encode()
    );

    let mut queue = vec![
        Piece::I,
        Piece::T,
        Piece::O,
        Piece::S,
        Piece::Z,
        Piece::L,
        Piece::J,
    ];
    queue.shuffle(&mut thread_rng());

    let mut fumen = Fumen::default();
    fumen.pages.pop();
    pcf::solve_pc(
        &queue,
        board,
        true,
        true,
        &AtomicBool::new(false),
        pcf::placeability::simple_srs_spins,
        |soln| common::add_placement_pages(&mut fumen, board, soln),
    );

    println!(
        "PCO Solutions for sequence {:?}: http://fumen.zui.jp/?{}",
        &queue[..4],
        fumen.encode()
    );
}
