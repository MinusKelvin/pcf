pub mod combination;

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

#[derive(Copy, Clone, Debug, Eq)]
pub struct PieceSequence {
    seq: [Piece; 11],
    count: u8
}

impl PieceSequence {
    /// For empty hold sequences, interpret this as (next, hold).
    /// For nonempty hold sequences, interpret this as (hold, next).
    pub fn peek_next(&self) -> (Piece, Piece) {
        if self.count < 2 {
            panic!("Not enough pieces in the sequence to know what can be placed next");
        }
        (self.seq[self.count as usize - 1], self.seq[self.count as usize - 2])
    }

    pub fn remove_first(&mut self) {
        self.count -= 1;
    }

    pub fn remove_second(&mut self) {
        self.seq[self.count as usize - 2] = self.seq[self.count as usize - 1];
        self.count -= 1;
    }

    pub fn to_set(&self) -> PieceSet {
        let mut set = PieceSet::default();
        for i in 0..self.count {
            set = set.with(self.seq[i as usize]);
        }
        set
    }

    pub fn to_index(&self) -> usize {
        let mut index = 0;
        for i in 0..self.count as usize {
            index *= 7;
            index += self.seq[i] as usize;
        }
        index
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }
}

impl std::iter::Iterator for PieceSequence {
    type Item = Piece;

    fn next(&mut self) -> Option<Piece> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            Some(self.seq[self.count as usize])
        }
    }
}

impl std::iter::FromIterator<Piece> for PieceSequence {
    fn from_iter<T: IntoIterator<Item=Piece>>(iter: T) -> Self {
        let mut seq = [Piece::S; 11];
        let mut count = 0;
        for p in iter.into_iter().take(11) {
            seq[count as usize] = p;
            count += 1;
        }
        seq.rotate_right(11 - count as usize);
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

use data::PieceState;
include!(concat!(env!("OUT_DIR"), "/data.rs"));