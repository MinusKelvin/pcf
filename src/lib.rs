mod combination;
pub mod placeability;
mod solve;

pub use combination::*;
pub use solve::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Piece {
    S, Z, J, L, T, O, I
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<usize> for Piece {
    fn from(i: usize) -> Piece {
        PIECES[i]
    }
}

pub const PIECES: [Piece; 7] = [
    Piece::S, Piece::Z, Piece::J, Piece::L, Piece::T, Piece::O, Piece::I
];

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PieceSet(pub [u8; 7]);

impl PieceSet {
    pub fn without(mut self, p: Piece) -> PieceSet {
        self.0[p as usize] -= 1;
        self
    }

    pub fn contains(self, p: Piece) -> bool {
        self.0[p as usize] != 0
    }

    pub fn with(mut self, p: Piece) -> PieceSet {
        self.0[p as usize] += 1;
        self
    }

    fn candidate_index(self) -> Result<u32, ()> {
        let mut index = 0;
        for &count in &self.0 {
            if count > 4 {
                return Err(())
            }
            index = index * 5 + count as u32;
        }
        Ok(index)
    }

    fn from_candidate_index(mut index: u32) -> Option<PieceSet> {
        let mut this = PieceSet::default();
        for count in this.0.iter_mut().rev() {
            *count = (index % 5) as u8;
            index /= 5;
        }
        if index == 0 {
            Some(this)
        } else {
            None
        }
    }
}

impl Default for PieceSet {
    fn default() -> Self {
        PieceSet([0; 7])
    }
}

impl std::fmt::Display for PieceSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const NAMES: [char; 7] = ['S', 'Z', 'J', 'L', 'T', 'O', 'I'];
        for i in 0..7 {
            for _ in 0..self.0[i] {
                write!(f, "{}", NAMES[i])?;
            }
        }
        Ok(())
    }
}

impl std::iter::FromIterator<Piece> for PieceSet {
    fn from_iter<T: IntoIterator<Item=Piece>>(iter: T) -> Self {
        let mut this = PieceSet::default();
        for piece in iter {
            this = this.with(piece)
        }
        this
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn filled(height: usize) -> BitBoard {
        BitBoard((1 << height*10) - 1)
    }

    pub fn combine(self, other: Self) -> Self {
        BitBoard(self.0 | other.0)
    }

    pub fn remove(self, other: Self) -> Self {
        BitBoard(self.0 & !other.0)
    }

    pub fn overlaps(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    pub fn cell_filled(self, x: usize, y: usize) -> bool {
        self.0 & 1 << x+y*10 != 0
    }

    pub fn leftmost_empty_column(self, height: usize) -> usize {
        // start with completely filled row
        let mut collapsed = (1 << 10) - 1;
        for i in 0..height {
            collapsed &= self.0 >> i*10;
        }
        // collapsed has a 0 wherever there's an empty cell in rows 0..height
        // so to find the x of first one, we need only count the number of 1s before it
        (!collapsed).trailing_zeros() as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Placement {
    pub kind: PieceState,
    pub x: u8
}

impl Placement {
    pub fn board(self) -> BitBoard {
        BitBoard(self.kind.board().0 << self.x)
    }

    fn placeable(self, on: BitBoard) -> bool {
        let mut hurdled_lines = 0;
        for i in (1..5).rev() {
            hurdled_lines <<= 10;
            if self.kind.hurdles() & 1 << i != 0 {
                hurdled_lines |= (1 << 10) - 1;
            }
        }
        // shift by 10 since the above loop skips the bottom row since it can't be hurdled
        if BitBoard(hurdled_lines << 10).remove(on) != BitBoard(0) {
            // hurdled lines not filled means the hurdled placement is impossible
            false
        } else {
            self.kind.grounded() || on.overlaps(BitBoard(self.kind.below_mask().0 << self.x))
        }
    }

    fn harddrop_mask(self) -> BitBoard {
        BitBoard(self.kind.harddrop_mask().0 << self.x)
    }
}

use data::PieceState;
include!(concat!(env!("OUT_DIR"), "/data.rs"));