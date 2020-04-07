use fumen::{ Fumen, CellColor };
use pcf::{ BitBoard, PieceSet, Piece, SearchStatus };
use rand::prelude::*;

mod common;

fn main() {
    let board = BitBoard(0b1100001111_1110001111_1111001111_1110001111);
    let pieces = PieceSet::default().with(Piece::I).with(Piece::T).with(Piece::O)
        .with(Piece::S).with(Piece::Z).with(Piece::L).with(Piece::J);

    let mut fumen = Fumen::default();
    common::blit(&mut fumen.pages[0], board, CellColor::Grey);

    pcf::find_combinations(pieces, board, 4, |combo| {
        let mut page = fumen.pages[0].clone();
        common::draw_placements(&mut page, &combo);
        fumen.pages.push(page);
        SearchStatus::Continue
    });

    println!("Combinatorial PCO Solutions: http://fumen.zui.jp/?{}", fumen.encode());

    let mut queue = vec![
        Piece::I, Piece::T, Piece::O, Piece::S, Piece::Z, Piece::L, Piece::J
    ];
    queue.shuffle(&mut thread_rng());

    let mut fumen = Fumen::default();
    common::blit(&mut fumen.pages[0], board, CellColor::Grey);
    for soln in pcf::solve_pc(&queue, board, true, true, pcf::placeability::always) {
        let mut page = fumen.pages[0].clone();
        common::draw_placements(&mut page, &soln);
        fumen.pages.push(page);
    }

    println!(
        "PCO Solutions for sequence {:?}: http://fumen.zui.jp/?{}", &queue[..4], fumen.encode()
    );
}