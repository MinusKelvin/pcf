use fumen::{CellColor, Fumen, Page};
use pcf::{BitBoard, Piece, Placement};

pub fn draw_placements(page: &mut Page, placements: &[Placement]) {
    for placement in placements {
        blit(
            page,
            placement.board(),
            pcf_piece_to_fumen_piece(placement.kind.piece()).into(),
        );
    }
}

pub fn add_placement_pages(fumen: &mut Fumen, mut on: BitBoard, placements: &[Placement]) {
    let mut iter = placements.iter();
    if let Some(placement) = iter.next() {
        let mut page = Page::default();
        blit(&mut page, on, CellColor::Grey);
        let srs = placement.srs_piece(on).into_iter().next().unwrap();
        page.piece = Some(fumen::Piece {
            kind: pcf_piece_to_fumen_piece(srs.piece),
            rotation: pcf_rot_to_fumen_rot(srs.rotation),
            x: srs.x as u32,
            y: srs.y as u32,
        });
        fumen.pages.push(page);
        on = on.combine(placement.board());
    }
    for placement in iter {
        let page = fumen.add_page();
        let srs = placement.srs_piece(on).into_iter().next().unwrap();
        page.piece = Some(fumen::Piece {
            kind: pcf_piece_to_fumen_piece(srs.piece),
            rotation: pcf_rot_to_fumen_rot(srs.rotation),
            x: srs.x as u32,
            y: srs.y as u32,
        });
        on = on.combine(placement.board());
    }
}

pub fn blit(page: &mut Page, board: BitBoard, color: CellColor) {
    for y in 0..6 {
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

pub fn pcf_rot_to_fumen_rot(rot: pcf::Rotation) -> fumen::RotationState {
    match rot {
        pcf::Rotation::North => fumen::RotationState::North,
        pcf::Rotation::South => fumen::RotationState::South,
        pcf::Rotation::West => fumen::RotationState::West,
        pcf::Rotation::East => fumen::RotationState::East,
    }
}
