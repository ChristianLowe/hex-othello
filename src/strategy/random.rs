use crate::strategy::Strategy;
use crate::board::Board;
use rand::seq::SliceRandom;

pub struct Random {}

impl Random {
    pub fn new() -> Random { Random {} }
}

impl Strategy for Random {
    fn choose_move(&self, _board: Board, board_indexes: Vec<u8>) -> u8 {
        *board_indexes.choose(&mut rand::thread_rng()).unwrap()
    }
}
