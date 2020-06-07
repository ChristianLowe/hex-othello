use crate::board::Board;
use crate::config::Config;
use crate::strategy::random::Random;

mod random;

pub trait Strategy {
    fn choose_move(&self, board: Board, board_indexes: Vec<u8>) -> u8;
}

pub struct StrategyFactory {}

impl StrategyFactory {
    pub fn from(config: &Config) -> Box<dyn Strategy> {
        if config.run_params.strategy.eq("random") { Box::new(Random::new()) } else {panic!()}
    }
}
