use crate::*;

pub fn solve_pc(
    queue: &[Piece],
    board: BitBoard,
    hold_allowed: bool,
    unique: bool,
    placability_judge: impl Fn(BitBoard, Placement) -> bool
) -> Vec<Vec<Placement>> {
    let mut lowest_height = 0;
    for y in 0..6 {
        if board.0 >> y*10 & (1 << 10) - 1 != 0 {
            lowest_height = y+1;
        }
    }
    let unfilled = 10*lowest_height - board.0.count_ones() as usize;
    if unfilled % 2 != 0 {
        // can never fill an odd number of cells
        return vec![];
    } else if unfilled % 4 != 0 {
        // need to fill an extra line to get a PC
        lowest_height += 1;
    }
    if lowest_height == 0 {
        lowest_height = 2;
    }

    let mut results = vec![];
    for height in (lowest_height..=6).step_by(2) {
        let unfilled = 10*height - board.0.count_ones() as usize;
        let pieces = unfilled / 4;
        if queue.len() < pieces {
            break
        }
        let queue: PieceSequence = queue.iter().copied().take(pieces + hold_allowed as usize).collect();

        find_combinations(queue.to_set(), board, height, |combo| {
            solve(
                &mut results,
                &mut vec![], queue, board, &mut combo.to_vec(),
                hold_allowed, unique, &mut false, &placability_judge
            );
            SearchStatus::Continue
        });

        if !results.is_empty() {
            break
        }
    }
    results
}

pub fn solve_pc_at_height(
    queue: &[Piece],
    board: BitBoard,
    hold_allowed: bool,
    unique: bool,
    height: usize,
    placability_judge: impl Fn(BitBoard, Placement) -> bool
) -> Vec<Vec<Placement>> {
    let unfilled = 10*height - board.0.count_ones() as usize;
    if unfilled % 4 != 0 {
        // can only fill a multiple of 4 empty cells with tetrominos
        return vec![];
    }
    let pieces = unfilled / 4 + hold_allowed as usize;
    let queue: PieceSequence = queue.iter().copied().take(pieces).collect();

    let mut results = vec![];
    find_combinations(queue.to_set(), board, height, |combo| {
        solve(
            &mut results,
            &mut vec![], queue, board, &mut combo.to_vec(),
            hold_allowed, unique, &mut false, &placability_judge
        );
        SearchStatus::Continue
    });
    results
}

pub fn solve_placement_combination(
    queue: &[Piece],
    board: BitBoard,
    combination: &[Placement],
    hold_allowed: bool,
    unique: bool,
    placability_judge: impl Fn(BitBoard, Placement) -> bool
) -> Vec<Vec<Placement>> {
    let mut results = vec![];
    solve(
        &mut results,
        &mut vec![], queue.iter().copied().collect(), board, &mut combination.to_vec(),
        hold_allowed, unique, &mut false, &placability_judge
    );
    results
}

fn solve(
    results: &mut Vec<Vec<Placement>>,
    permutation: &mut Vec<Placement>,
    queue: PieceSequence,
    board: BitBoard,
    remaining: &mut Vec<Placement>,
    hold_allowed: bool,
    unique: bool, found: &mut bool,
    placability_judge: &impl Fn(BitBoard, Placement) -> bool
) {
    if remaining.is_empty() {
        results.push(permutation.clone());
        if unique { *found = true }
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
                results,
                permutation, new_queue, new_board, remaining,
                hold_allowed, unique, found, placability_judge
            );

            permutation.pop();
            remaining.push(placement);
            let last_index = remaining.len() - 1;
            remaining.swap(i, last_index);
            // the above restores the original state of the remaining vec

            if *found { break }
        }
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