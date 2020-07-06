use crate::config::Config;
use crate::connector::Connector;
use std::time::Duration;
use std::{thread, io};
use board_em_api::{WebClient, WebGame};
use std::io::Write;

pub struct BoardEm {
    web_client: WebClient,
    game_id: String,
    poll_duration: Duration,
    move_list: Vec<String>,
}

impl BoardEm {
    fn get_latest_web_game(&self) -> WebGame {
        print!("Getting latest web game... ");
        io::stdout().flush().unwrap();
        loop {
            let web_game = self.web_client.get_latest_web_game(&self.game_id);
            if web_game.is_err() {
                let err = web_game.unwrap_err();
                println!("Error making web call: {}", err);
                thread::sleep(self.poll_duration);
                continue;
            }

            println!("ok!");
            return web_game.unwrap();
        }
    }
}

impl Connector for BoardEm {
    fn from_config(config: Config) -> Self where Self: Sized {
        let config = config.connector.board_em;
        BoardEm {
            web_client: WebClient::from_hostname(config.hostname),
            game_id: config.game_id,
            poll_duration: Duration::from_millis(config.poll_rate),
            move_list: Vec::new()
        }
    }

    fn get_next_move_list(&mut self) -> Vec<String> {
        loop {
            let web_game = self.get_latest_web_game();
            if web_game.moves.len() == self.move_list.len() {
                thread::sleep(self.poll_duration);
                continue;
            }

            self.move_list = web_game.moves.clone();
            return web_game.moves;
        }
    }

    fn submit_move(&mut self, new_move: &String) {
        loop {
            let web_game = self.get_latest_web_game();
            let response = self.web_client.submit_move(&web_game, new_move);
            if response.is_err() {
                let err = response.unwrap_err();
                println!("Error making web call: {}", err);
                thread::sleep(self.poll_duration);
                continue;
            }

            let response = response.unwrap();
            if response.status() != 200 {
                println!("Error, response is status {}", response.status());
                thread::sleep(self.poll_duration);
                continue;
            }

            self.move_list.push(new_move.clone());
            println!("Response: {:?}", response);
            return;
        }
    }
}
