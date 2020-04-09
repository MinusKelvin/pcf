use crate::*;

pub fn find_combinations(
    piece_set: PieceSet, field: BitBoard, height: usize,
    mut combo_consumer: impl FnMut(&[Placement]) -> SearchStatus
) {
    find_combos(
        &mut vec![],
        field,
        BitBoard::filled(height),
        piece_set,
        height,
        &mut combo_consumer
    );
}

fn find_combos(
    placements: &mut Vec<Placement>,
    board: BitBoard,
    inverse_placed: BitBoard,
    piece_set: PieceSet,
    height: usize,
    combo_consumer: &mut impl FnMut(&[Placement]) -> SearchStatus
) -> Option<()> {
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
            return Some(());
        }
    }

    if board == BitBoard::filled(height) {
        // I would like to just overload the ? operator for SearchStatus, but try_trait is unstable
        match combo_consumer(placements) {
            SearchStatus::Continue => Some(()),
            SearchStatus::Abort => None
        }
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

        for &piece_state in crate::data::PIECE_STATES_FOR_HEIGHT[height-1] {
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
                placements,
                new_board,
                new_inverse_placed,
                piece_set.without(piece_state.piece()),
                height,
                combo_consumer
            )?;
            placements.pop();
        }

        Some(())
    }
}
