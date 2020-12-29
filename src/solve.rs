use crate::*;
use std::sync::atomic::AtomicBool;

pub fn solve_pc(
    queue: &[Piece],
    board: BitBoard,
    hold_allowed: bool,
    unique: bool,
    abort: &AtomicBool,
    placeability_judge: impl Fn(BitBoard, Placement) -> bool,
    mut pc_consumer: impl FnMut(&[Placement])
) {
    solve_pc_prep(queue, board, hold_allowed, |queue, height| {
        let mut found = false;
        find_combinations(queue.to_set(), board, abort, height, |combo| {
            solve_placement_combo(
                queue, board, combo,
                hold_allowed, unique, &placeability_judge,
                |soln| {
                    found = true;
                    pc_consumer(soln)
                }
            )
        });
        found
    });
}

pub fn solve_pc_mt(
    queue: &[Piece],
    board: BitBoard,
    hold_allowed: bool,
    unique: bool,
    abort: &AtomicBool,
    placeability_judge: impl Fn(BitBoard, Placement) -> bool + Sync,
    pc_consumer: impl FnMut(&[Placement]) + Clone + Send
) {
    let placeability_judge = &placeability_judge;
    solve_pc_prep(queue, board, hold_allowed, |queue, height| {
        let found = &std::sync::atomic::AtomicBool::new(false);
        let mut pc_consumer = pc_consumer.clone();
        find_combinations_mt(queue.to_set(), board, abort, height, move |combo| {
            solve_placement_combo(
                queue, board, combo,
                hold_allowed, unique, placeability_judge,
                |soln| {
                    found.store(true, std::sync::atomic::Ordering::Release);
                    pc_consumer(soln)
                }
            )
        });
        found.load(std::sync::atomic::Ordering::Acquire)
    });
}

fn solve_pc_prep(
    queue: &[Piece],
    board: BitBoard,
    hold_allowed: bool,
    mut do_solve: impl FnMut(PieceSequence, usize) -> bool
) {
    let mut lowest_height = 0;
    for y in 0..6 {
        if board.0 >> y*10 & (1 << 10) - 1 != 0 {
            lowest_height = y+1;
        }
    }
    let unfilled = 10*lowest_height - board.0.count_ones() as usize;
    if unfilled % 2 != 0 {
        // can never fill an odd number of cells
        return;
    } else if unfilled % 4 != 0 {
        // need to fill an extra line to get a PC
        lowest_height += 1;
    }
    if lowest_height == 0 {
        lowest_height = 2;
    }

    for height in (lowest_height..=6).step_by(2) {
        let unfilled = 10*height - board.0.count_ones() as usize;
        let pieces = unfilled / 4;
        if queue.len() < pieces {
            break
        }
        let queue: PieceSequence = queue.iter().copied()
            .take(pieces + hold_allowed as usize)
            .collect();

        if do_solve(queue, height) {
            break
        }
    }
}

pub fn solve_placement_combination(
    queue: &[Piece],
    board: BitBoard,
    combination: &[Placement],
    hold_allowed: bool,
    unique: bool,
    abort: &AtomicBool,
    placability_judge: impl Fn(BitBoard, Placement) -> bool,
    pc_consumer: impl FnMut(&[Placement])
) {
    solve_placement_combo(
        queue.iter().copied().collect(), board, combination,
        hold_allowed, unique, placability_judge, pc_consumer
    );
}

fn solve_placement_combo(
    queue: PieceSequence,
    board: BitBoard,
    combination: &[Placement],
    hold_allowed: bool,
    unique: bool,
    placability_judge: impl Fn(BitBoard, Placement) -> bool,
    mut pc_consumer: impl FnMut(&[Placement])
) {
    let mut combo = ArrayVec::new();
    combo.try_extend_from_slice(combination).unwrap();
    solve(
        &mut ArrayVec::new(), queue, board, &mut combo,
        hold_allowed, unique, &placability_judge, &mut pc_consumer
    );
}

fn solve(
    permutation: &mut ArrayVec<[Placement; 15]>,
    queue: PieceSequence,
    board: BitBoard,
    remaining: &mut ArrayVec<[Placement; 15]>,
    hold_allowed: bool,
    unique: bool,
    placability_judge: &impl Fn(BitBoard, Placement) -> bool,
    pc_consumer: &mut impl FnMut(&[Placement])
) -> Option<()> {
    if remaining.is_empty() {
        pc_consumer(permutation);
        if unique {
            None
        } else {
            Some(())
        }
    } else {
        // we have the invariant that the range 0..n remains the same from one iteration to the next
        // we do still have to mutate the remaining vec in ways that the compiler can only view as
        // aribtrary, though, so we can't actually use normal iteration here.
        for i in 0..remaining.len() {
            let placement = remaining[i];

            if !queue.is_next(placement.kind.piece(), hold_allowed) {
                // can't place this placement since it's neither next nor obtainable through hold
                continue
            }
            if !placement.placeable(board) {
                // unplaceable placement obviously can't come next
                continue
            }
            if !placability_judge(board, placement) {
                // the judge has determined that you can't place that piece there
                // e.g. unreachable using Super Rotation System rules
                continue
            }

            let new_board = board.combine(placement.board());
            let mut new_queue = queue;
            new_queue.remove(placement.kind.piece());
            remaining.swap_remove(i);
            permutation.push(placement);

            solve(
                permutation, new_queue, new_board, remaining,
                hold_allowed, unique, placability_judge, pc_consumer
            )?;

            permutation.pop();
            remaining.push(placement);
            let last_index = remaining.len() - 1;
            remaining.swap(i, last_index);
            // the above restores the original state of the remaining vec
        }

        Some(())
    }
}

#[derive(Copy, Clone, Debug, Eq)]
struct PieceSequence {
    seq: [Piece; 16],
    count: u8
}

impl PieceSequence {
    fn is_next(&self, piece: Piece, hold_allowed: bool) -> bool {
        self.count != 0 && (
            self.seq[self.count as usize - 1] == piece ||
            hold_allowed && self.count != 1 && self.seq[self.count as usize - 2] == piece
        )
    }

    fn remove(&mut self, piece: Piece) {
        if self.seq[self.count as usize - 1] != piece {
            self.seq[self.count as usize - 2] = self.seq[self.count as usize - 1];
        }
        self.count -= 1;
    }

    fn to_set(&self) -> PieceSet {
        let mut set = PieceSet::default();
        for i in 0..self.count {
            set = set.with(self.seq[i as usize]);
        }
        set
    }
}

impl std::iter::FromIterator<Piece> for PieceSequence {
    fn from_iter<T: IntoIterator<Item=Piece>>(iter: T) -> Self {
        let mut seq = [Piece::S; 16];
        let mut count = 0;
        for p in iter.into_iter().take(16) {
            seq[count as usize] = p;
            count += 1;
        }
        seq.rotate_right(16 - count as usize);
        seq.reverse();
        PieceSequence {
            seq, count
        }
    }
}

impl std::cmp::PartialEq for PieceSequence {
    fn eq(&self, other: &Self) -> bool {
        if self.count != other.count {
            return false
        }
        for i in 0..self.count as usize {
            if self.seq[i] != other.seq[i] {
                return false
            }
        }
        true
    }
}

impl std::hash::Hash for PieceSequence {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        h.write_u8(self.count);
        for i in 0..self.count as usize {
            self.seq[i].hash(h);
        }
    }
}