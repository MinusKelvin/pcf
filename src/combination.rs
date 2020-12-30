use crate::*;
use std::sync::atomic::{ AtomicBool, Ordering };

pub fn find_combinations(
    piece_set: PieceSet, board: BitBoard, abort: &AtomicBool, height: usize,
    mut combo_consumer: impl FnMut(&[Placement])
) {
    find_combos_st(
        &mut vec![],
        board,
        BitBoard::filled(height),
        piece_set,
        abort,
        height,
        &mut combo_consumer
    );
}

pub fn find_combinations_mt(
    piece_set: PieceSet, board: BitBoard, abort: &AtomicBool, height: usize,
    combo_consumer: impl FnMut(&[Placement]) + Clone + Send
) {
    rayon::scope(|scope|
        find_combos_mt(
            scope, vec![], board, BitBoard::filled(height), piece_set,
            abort, height, 0, combo_consumer
        )
    );
}

fn find_combos_st(
    placements: &mut Vec<Placement>,
    board: BitBoard,
    inverse_placed: BitBoard,
    piece_set: PieceSet,
    abort: &AtomicBool,
    height: usize,
    combo_consumer: &mut impl FnMut(&[Placement])
) {
    find_combos(
        board, inverse_placed, piece_set, abort, height,
        |placement, board, inverse_placed, piece_set| {
            placements.push(placement);
            if has_cyclic_dependency(inverse_placed, placements, height) {
                
            } else if board == BitBoard::filled(height) {
                combo_consumer(placements);
            } else {
                find_combos_st(
                    placements, board, inverse_placed, piece_set, abort, height, combo_consumer
                )
            };
            placements.pop();
        }
    )
}

fn find_combos_mt<'s>(
    scope: &rayon::Scope<'s>,
    mut placements: Vec<Placement>,
    board: BitBoard,
    inverse_placed: BitBoard,
    piece_set: PieceSet,
    abort: &'s AtomicBool,
    height: usize, recursions: usize,
    mut combo_consumer: impl FnMut(&[Placement]) + Clone + Send + 's
) {
    if recursions >= 3 {
        find_combos_st(
            &mut placements, board, inverse_placed, piece_set, abort, height, &mut combo_consumer
        );
    } else {
        find_combos(
            board, inverse_placed, piece_set, abort, height,
            |placement, board, inverse_placed, piece_set| {
                placements.push(placement);
                if has_cyclic_dependency(inverse_placed, &placements, height) {

                } else if board == BitBoard::filled(height) {
                    combo_consumer(&placements);
                } else {
                    let p = placements.clone();
                    let c = combo_consumer.clone();
                    scope.spawn(move |scope| find_combos_mt(
                        scope, p, board, inverse_placed, piece_set, abort, height, recursions+1, c
                    ));
                }
                placements.pop();
            }
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
    mut next: impl FnMut(Placement, BitBoard, BitBoard, PieceSet)
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
            continue
        }
        for &piece_state in crate::data::PIECE_STATES_FOR_HEIGHT_AND_PIECE[height-1][piece as usize] {
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

            // Check if we should abort the search
            if abort.load(Ordering::Relaxed) {
                return;
            }

            let new_board = piece_board.combine(board);
            let new_inverse_placed = inverse_placed.remove(piece_board);

            next(placement, new_board, new_inverse_placed, piece_set.without(piece));
        }
    }
}


/// Check that no cyclic placement dependency exists. e.g. An S hurdles row 1, and an O hurdles
/// row 2. To place the O, the S must be used to clear a line first. To place the S, the O must
/// be used to clear a line first. Obviously, these dependencies cannot be satisfied.
#[inline(always)]
fn has_cyclic_dependency(
    inverse_placed: BitBoard, placements: &[Placement], height: usize
) -> bool {
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
    supported != BitBoard::filled(height)
}