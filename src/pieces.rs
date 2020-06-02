
use crate::board_index::*;
use crate::direction::Direction;

use std::fmt;
use std::fmt::Formatter;
use std::ops::{BitOr, BitOrAssign, BitAnd, BitAndAssign, BitXor, BitXorAssign, Not};

#[derive(Copy, Clone, Debug)]
pub struct Pieces {
    bits: u64
}

impl Pieces {
    pub fn new() -> Pieces {
        Pieces { bits: 0 }
    }

    pub fn from_index(index: BoardIndex) -> Pieces {
        let mut pieces = Pieces::new();
        pieces.place_piece(index);
        pieces
    }

    pub fn place_piece(&mut self, index: BoardIndex) {
        self.bits |= 1 << index as u64;
    }

    // pub fn has_piece(&self, index: BoardIndex) -> bool {
    //     self.bits & (1 << index) != 0
    // }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn piece_count(&self) -> u32 {
        self.bits.count_ones()
    }

    pub fn board_indexes(&self) -> Vec<BoardIndex> {
        let mut board_indexes = Vec::new();

        let mut bits = self.bits;
        while bits != 0 {
            let index = bits.trailing_zeros() as BoardIndex;
            board_indexes.push(index );
            bits ^= 1u64 << index as u64;
        }

        board_indexes
    }

    pub fn with_slide(&self, direction: &Direction) -> Pieces {
        let slid_bits = match direction {
            Direction::NorthWest    => self.bits << 9,
            Direction::North        => self.bits << 8,
            Direction::NorthEast    => self.bits << 7,
            Direction::East         => self.bits >> 1,
            Direction::SouthEast    => self.bits >> 9,
            Direction::South        => self.bits >> 8,
            Direction::SouthWest    => self.bits >> 7,
            Direction::West         => self.bits << 1,
        };

        Pieces { bits: slid_bits & direction.mask() }
    }
}

impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut piece_names = Vec::new();

        let mut bits = self.bits;
        for index in 0..64 {
            let has_piece = (bits & 1) == 1;
            if has_piece {
                piece_names.push(BoardIndex::index_to_piece_name(index))
            }

            bits >>= 1
        }

        write!(f, "{}", piece_names.join(", "))
    }
}

impl BitOr for Pieces {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Pieces { bits: self.bits | rhs.bits }
    }
}

impl BitOrAssign for Pieces {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits
    }
}

impl BitAnd for Pieces {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Pieces { bits: self.bits & rhs.bits }
    }
}

impl BitAndAssign for Pieces {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits
    }
}

impl BitXor for Pieces {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Pieces { bits: self.bits ^ rhs.bits }
    }
}

impl BitXorAssign for Pieces {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits
    }
}

impl Not for Pieces {
    type Output = Self;

    fn not(self) -> Self {
        Pieces { bits: !self.bits }
    }
}