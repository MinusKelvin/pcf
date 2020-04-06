use fumen::{ Fumen, CellColor };
use pc_finder::{ BitBoard, PieceSet, Piece, combination::find_combinations };

mod common;

fn main() {
    let board = BitBoard(0b1100001111_1110001111_1111001111_1110001111);
    let pieces = PieceSet::default().with(Piece::I).with(Piece::T).with(Piece::O)
        .with(Piece::S).with(Piece::Z).with(Piece::L).with(Piece::J);

    let mut fumen = Fumen::default();
    common::blit(&mut fumen.pages[0], board, CellColor::Grey);

    for combo in find_combinations(pieces, board, 4) {
        let mut page = fumen.pages[0].clone();
        common::draw_placements(&mut page, &combo);
        fumen.pages.push(page);
    }

    println!("Combinatorial PCO Solutions: http://fumen.zui.jp/?{}", fumen.encode());
}