
use reqwest::Error;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebGame {
    pub id: String,
    pub moves: Vec<String>,
    pub board: Vec<Vec<u8>>,
    pub render: String,
}

pub struct WebClient {
    client: Client,
    id: String
}

impl WebClient {
    pub fn from(game_id: String) -> WebClient {
        WebClient {
            client: Client::new(),
            id: game_id
        }
    }

    pub fn get_latest_web_game(&self) -> Result<WebGame, Box<dyn std::error::Error>> {
        let request_url = format!("https://demo.mattmerr.com/api/games/{}", self.id);
        let response = reqwest::blocking::get(&request_url)?;
        let web_game: WebGame = response.json()?;
        Result::Ok(web_game)
    }

    pub fn submit_move(&self, web_game: &WebGame, next_move: &String) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let move_number = web_game.moves.len();
        let request_url = format!("https://demo.mattmerr.com/api/games/{}/moves/{}", self.id, move_number);
        self.client.post(request_url.as_str()).json(next_move).send()
    }
}