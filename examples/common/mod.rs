use pc_finder::{ BitBoard, Placement, Piece };
use fumen::{ Page, CellColor };

pub fn draw_placements(page: &mut Page, placements: &[Placement]) {
    for placement in placements {
        blit(page, placement.board(), pcf_piece_to_fumen_piece(placement.kind.piece()).into());
    }
}

pub fn blit(page: &mut Page, board: BitBoard, color: CellColor) {
    for y in 0..4 {
        for x in 0..10 {
            if board.cell_filled(x, y) {
                page.field[y][x] = color;
            }
        }
    }
}

pub fn pcf_piece_to_fumen_piece(piece: Piece) -> fumen::PieceType {
    match piece {
        Piece::I => fumen::PieceType::I,
        Piece::T => fumen::PieceType::T,
        Piece::O => fumen::PieceType::O,
        Piece::S => fumen::PieceType::S,
        Piece::Z => fumen::PieceType::Z,
        Piece::L => fumen::PieceType::L,
        Piece::J => fumen::PieceType::J,
    }
}