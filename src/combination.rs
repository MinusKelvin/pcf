use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Placement {
    pub kind: PieceState,
    pub x: u8
}

impl Placement {
    pub fn board(self) -> BitBoard {
        BitBoard(self.kind.board().0 << self.x)
    }

    pub fn placeable(self, on: BitBoard) -> bool {
        self.kind.grounded() || on.overlaps(BitBoard(self.kind.below_mask().0 << self.x))
    }

    pub fn harddrop_mask(self) -> BitBoard {
        BitBoard(self.kind.harddrop_mask().0 << self.x)
    }
}

pub fn find_combinations(
    piece_set: PieceSet, field: BitBoard, height: usize
) -> Vec<Vec<Placement>> {
    let mut combinations = vec![];
    find_combos(
        &mut combinations,
        &mut vec![],
        field,
        BitBoard::filled(height),
        piece_set,
        height
    );
    combinations
}

fn find_combos(
    combos: &mut Vec<Vec<Placement>>,
    placements: &mut Vec<Placement>,
    board: BitBoard,
    inverse_placed: BitBoard,
    piece_set: PieceSet,
    height: usize
) {
    // Check that no cyclic placement dependency exists. e.g. An S hurdles row 1, and an O hurdles
    // row 2. To place the O, the S must be used to clear a line first. To place the S, the O must
    // be used to clear a line first. Obviously, these dependencies cannot be satisfied.
    {
        // Initially filled spots of the field obviously provide support, but the empty parts
        // that haven't been filled by anything? Yes actually, since we could choose a set of
        // placements that fill in the empty cells that are all supported. At the very least, we
        // can't exclude the possibility since we need to take a conservative approach here.
        let mut supported = inverse_placed;

        // O(n^2) loop is kinda yikes, but the whole find_combinations routine is O(n!) so...
        'place: loop {
            for &p in &*placements {
                let piece_board = p.board();

                if supported.overlaps(piece_board) {
                    // this basically checks if we've already placed p on the board
                    continue
                }

                let mut hurdled_lines = 0;
                for i in (1..5).rev() {
                    hurdled_lines <<= 10;
                    if p.kind.hurdles() & 1 << i != 0 {
                        hurdled_lines |= (1 << 10) - 1;
                    }
                }
                // shift by 10 since the above loop skips the bottom row since it can't be hurdled
                if BitBoard(hurdled_lines << 10).remove(supported) != BitBoard(0) {
                    // hurdled lines not filled means hurdled placement not supported
                    continue
                }

                if p.placeable(supported) {
                    // supported placement
                    supported = supported.combine(piece_board);
                    continue 'place;
                }
            }
            // can't place any more supported pieces
            break;
        }

        // This is reached when all placements that can be supported are placed. If there are
        // any holes in the supported field, then we know that there are some placements that
        // have a cyclic dependency and therefore this combination can't ever be placed.
        if supported != BitBoard::filled(height) {
            return;
        }
    }

    if board == BitBoard::filled(height) {
        combos.push(placements.clone());
    } else {
        let x = board.leftmost_empty_column(height);
        let mut y = 0;
        for i in 0..height {
            if !board.cell_filled(x, i) {
                y = i;
                break;
            }
        }
        let y = y;

        for &piece_state in crate::data::PIECE_STATES_FOR_HEIGHT[height] {
            if !piece_set.contains(piece_state.piece()) {
                // this piece can't be used again
                continue
            }
            if x + piece_state.width() as usize > 10 {
                // piece doesn't fit
                continue
            }
            
            let placement = Placement {
                kind: piece_state,
                x: x as u8
            };
            let piece_board = placement.board();

            if !piece_board.cell_filled(x, y) {
                // piece doesn't fill the cell we're trying to fill
                continue;
            }
            if piece_board.overlaps(board) {
                // can't place piece here
                continue;
            }

            let new_board = piece_board.combine(board);
            let new_inverse_placed = inverse_placed.remove(piece_board);

            placements.push(placement);
            find_combos(
                combos,
                placements,
                new_board,
                new_inverse_placed,
                piece_set.without(piece_state.piece()),
                height
            );
            placements.pop();
        }
    }
}
