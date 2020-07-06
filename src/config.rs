
use std::fs::File;
use std::io::Read;

use serde::{Deserialize};

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub run_params: RunConfig,
    pub connector: ConnectorConfig,
    pub strategy: StrategyConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct RunConfig {
    pub connector: String,
    pub strategy: String,
    pub ai_color: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ConnectorConfig {
    pub board_em: BoardEmConfig,
    pub ipc: IpcConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct BoardEmConfig {
    pub hostname: String,
    pub game_id: String,
    pub poll_rate: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct IpcConfig {
    pub game_id: String
}

#[derive(Clone, Deserialize, Debug)]
pub struct StrategyConfig {
    pub random: RandomConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct RandomConfig {
    pub seed: String,
}

impl Config {
    pub fn from(path: &str) -> Config {
        let mut config_toml = String::new();

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_)   => panic!("Config not found at location: {}", path),
        };

        file.read_to_string(&mut config_toml)
            .expect("Error while reading config");

        toml::from_str(config_toml.as_str()).unwrap()
    }
}
