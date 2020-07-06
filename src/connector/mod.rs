use crate::config::Config;
use crate::connector::board_em::BoardEm;
use crate::connector::ipc::Ipc;

mod board_em;
mod ipc;

pub trait Connector {
    fn from_config(config: Config) -> Self where Self: Sized;
    fn get_next_move_list(&mut self) -> Vec<String>;
    fn submit_move(&mut self, new_move: &String);
}

pub struct ConnectorFactory {}

impl ConnectorFactory {
    pub fn from(config: &Config) -> Box<dyn Connector> {
        let connector = config.run_params.connector.clone();
        let config = config.clone();

        if connector.eq_ignore_ascii_case("board_em") {
            Box::new(BoardEm::from_config(config))
        } else if connector.eq_ignore_ascii_case("ipc") {
            Box::new(Ipc::from_config(config))
        } else {
            panic!("Unknown/unsupported connector: {}", connector)
        }
    }
}
