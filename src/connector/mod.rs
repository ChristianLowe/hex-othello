use crate::config::Config;
use crate::connector::board_em::BoardEm;

mod board_em;

pub trait Connector {
    fn get_next_move_list(&mut self) -> Vec<String>;
    fn submit_move(&mut self, new_move: &String);
}

pub struct ConnectorFactory {}

impl ConnectorFactory {
    pub fn from(config: &Config) -> Box<dyn Connector> {
        if config.run_params.connector.eq("board_em") { Box::new(BoardEm::from(config.connector.board_em.clone())) } else {panic!()}
    }
}
