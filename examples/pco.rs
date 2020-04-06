use fumen::{ Fumen, Page, CellColor };
use pc_finder::{ BitBoard, PieceSet, Piece, combination::find_combinations };

fn main() {
    let mut fumen = Fumen::default();
    
    {
        use fumen::CellColor::{ Grey, Empty };
        fumen.pages[0].field[3] = [Grey, Grey, Grey, Grey,Empty,Empty,Empty,Empty, Grey, Grey];
        fumen.pages[0].field[2] = [Grey, Grey, Grey, Grey,Empty,Empty,Empty, Grey, Grey, Grey];
        fumen.pages[0].field[1] = [Grey, Grey, Grey, Grey,Empty,Empty, Grey, Grey, Grey, Grey];
        fumen.pages[0].field[0] = [Grey, Grey, Grey, Grey,Empty,Empty,Empty, Grey, Grey, Grey];
    }
    let bitboard = BitBoard(0b1100001111_1110001111_1111001111_1110001111);
    let pieces = PieceSet::default()
        .with(Piece::I).with(Piece::T).with(Piece::O)
        .with(Piece::S).with(Piece::Z).with(Piece::L).with(Piece::J);

    for combo in find_combinations(pieces, bitboard, 4) {
        let mut page = fumen.pages[0].clone();
        for placement in combo {
            let placement_board = placement.board();
            for y in 0..4 {
                for x in 0..10 {
                    if placement_board.cell_filled(x, y) {
                        page.field[y][x] = pcf_to_fumen(placement.kind.piece());
                    }
                }
            }
        }
        fumen.pages.push(page);
    }

    println!("Combinatorial PCO Solutions: http://fumen.zui.jp/?{}", fumen.encode());
}

fn pcf_to_fumen(piece: Piece) -> CellColor {
    match piece {
        Piece::I => CellColor::I,
        Piece::T => CellColor::T,
        Piece::O => CellColor::O,
        Piece::S => CellColor::S,
        Piece::Z => CellColor::Z,
        Piece::L => CellColor::L,
        Piece::J => CellColor::J,
    }
}