use crate::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn find_combinations(
    piece_set: PieceSet,
    board: BitBoard,
    abort: &AtomicBool,
    height: usize,
    mut combo_consumer: impl FnMut(&[Placement]),
) {
    find_combos_st(
        &mut vec![],
        board,
        BitBoard::filled(height),
        piece_set,
        abort,
        height,
        &mut combo_consumer,
    );
}

pub fn find_combinations_mt(
    piece_set: PieceSet,
    board: BitBoard,
    abort: &AtomicBool,
    height: usize,
    combo_consumer: impl FnMut(&[Placement]) + Clone + Send,
) {
    rayon::scope(|scope| {
        find_combos_mt(
            scope,
            vec![],
            board,
            BitBoard::filled(height),
            piece_set,
            abort,
            height,
            0,
            combo_consumer,
        )
    });
}

fn find_combos_st(
    placements: &mut Vec<Placement>,
    board: BitBoard,
    inverse_placed: BitBoard,
    piece_set: PieceSet,
    abort: &AtomicBool,
    height: usize,
    combo_consumer: &mut impl FnMut(&[Placement]),
) {
    find_combos(
        board,
        inverse_placed,
        piece_set,
        abort,
        height,
        |placement, board, inverse_placed, piece_set| {
            placements.push(placement);
            if has_cyclic_dependency(inverse_placed, placements, height) {
            } else if board == BitBoard::filled(height) {
                combo_consumer(placements);
            } else if !vertical_parity_ok(board, piece_set, height) {
            } else {
                find_combos_st(
                    placements,
                    board,
                    inverse_placed,
                    piece_set,
                    abort,
                    height,
                    combo_consumer,
                )
            };
            placements.pop();
        },
    )
}

fn find_combos_mt<'s>(
    scope: &rayon::Scope<'s>,
    mut placements: Vec<Placement>,
    board: BitBoard,
    inverse_placed: BitBoard,
    piece_set: PieceSet,
    abort: &'s AtomicBool,
    height: usize,
    recursions: usize,
    mut combo_consumer: impl FnMut(&[Placement]) + Clone + Send + 's,
) {
    if recursions >= 3 {
        find_combos_st(
            &mut placements,
            board,
            inverse_placed,
            piece_set,
            abort,
            height,
            &mut combo_consumer,
        );
    } else {
        find_combos(
            board,
            inverse_placed,
            piece_set,
            abort,
            height,
            |placement, board, inverse_placed, piece_set| {
                placements.push(placement);
                if has_cyclic_dependency(inverse_placed, &placements, height) {
                } else if board == BitBoard::filled(height) {
                    combo_consumer(&placements);
                } else if !vertical_parity_ok(board, piece_set, height) {
                } else {
                    let p = placements.clone();
                    let c = combo_consumer.clone();
                    scope.spawn(move |scope| {
                        find_combos_mt(
                            scope,
                            p,
                            board,
                            inverse_placed,
                            piece_set,
                            abort,
                            height,
                            recursions + 1,
                            c,
                        )
                    });
                }
                placements.pop();
            },
        );
    }
}

#[inline(always)]
fn find_combos(
    board: BitBoard,
    inverse_placed: BitBoard,
    piece_set: PieceSet,
    abort: &AtomicBool,
    height: usize,
    mut next: impl FnMut(Placement, BitBoard, BitBoard, PieceSet),
) {
    let x = board.leftmost_empty_column(height);
    let mut y = 0;
    for i in 0..height {
        if !board.cell_filled(x, i) {
            y = i;
            break;
        }
    }
    let y = y;

    for &piece in &PIECES {
        if !piece_set.contains(piece) {
            // this piece can't be used again
            continue;
        }
        for &piece_state in
            crate::data::PIECE_STATES_BY_HEIGHT_KIND_CELLY[height - 1][piece as usize][y]
        {
            if x + piece_state.width() as usize > 10 {
                // piece doesn't fit. array is sorted by width, so all future states fail this too.
                break;
            }

            let placement = Placement {
                kind: piece_state,
                x: x as u8,
            };
            let piece_board = placement.board();

            if piece_board.overlaps(board) {
                // can't place piece here
                continue;
            }

            // Check if we should abort the search
            if abort.load(Ordering::Acquire) {
                return;
            }

            let new_board = piece_board.combine(board);
            let new_inverse_placed = inverse_placed.remove(piece_board);

            next(
                placement,
                new_board,
                new_inverse_placed,
                piece_set.without(piece),
            );
        }
    }
}

/// Check that no cyclic placement dependency exists. e.g. An S hurdles row 1, and an O hurdles
/// row 2. To place the O, the S must be used to clear a line first. To place the S, the O must
/// be used to clear a line first. Obviously, these dependencies cannot be satisfied.
#[inline(always)]
fn has_cyclic_dependency(
    inverse_placed: BitBoard,
    placements: &[Placement],
    height: usize,
) -> bool {
    // Initially filled spots of the field obviously provide support, but the empty parts
    // that haven't been filled by anything? Yes actually, since we could choose a set of
    // placements that fill in the empty cells that are all supported. At the very least, we
    // can't exclude the possibility since we need to take a conservative approach here.
    let mut supports = inverse_placed;

    // O(n^2) loop is kinda yikes, but the whole find_combinations routine is O(n!) so...
    'place: loop {
        for &p in &*placements {
            let piece_board = p.board();

            if supports.overlaps(piece_board) {
                // this basically checks if we've already placed p on the board
                continue;
            }

            if p.supported_without_clears(supports) {
                // supported placement
                supports = supports.combine(piece_board);
                continue 'place;
            }
        }
        // can't place any more supported pieces
        break;
    }

    // This is reached when all placements that can be supported are placed. If there are
    // any holes in the supported field, then we know that there are some placements that
    // have a cyclic dependency and therefore this combination can't ever be placed.
    supports != BitBoard::filled(height)
}

/// Check that vertical parity can be corrected with the available pieces.
/// There are 4 pieces that can change vertical parity: L and J in any orientation change it by
/// 1, vertical T changes it by 1, and vertical I changes it by 2.
#[inline(always)]
fn vertical_parity_ok(board: BitBoard, remaining: PieceSet, height: usize) -> bool {
    let remaining_pieces = BitBoard::filled(height).remove(board).0.count_ones() / 4;

    // pieces that can be placed without changing vertical parity
    let available_non_lj = remaining.0[Piece::S as usize] as u32
        + remaining.0[Piece::Z as usize] as u32
        + remaining.0[Piece::T as usize] as u32
        + remaining.0[Piece::I as usize] as u32
        + remaining.0[Piece::O as usize] as u32;

    let even_columns = 0b0101010101_0101010101_0101010101_0101010101_0101010101_0101010101;
    let vertical_parity = (board.0 & even_columns)
        .count_ones()
        .abs_diff((board.0 & !even_columns).count_ones())
        / 2;

    // remaining potential for vertical parity to be changed
    let can_change = remaining.0[Piece::L as usize] as u32
        + remaining.0[Piece::J as usize] as u32
        + remaining.0[Piece::T as usize] as u32
        + 2 * remaining.0[Piece::I as usize] as u32;

    // vertical parity must change more than it could potentially be changed -> no solutions
    if vertical_parity > can_change {
        return false;
    }

    // number of vertical parity changes that must happen due to forced L/J pieces
    let must_change = remaining_pieces - available_non_lj.min(remaining_pieces);
    if can_change == must_change && (vertical_parity ^ must_change) & 1 != 0 {
        false
    } else {
        true
    }
}
