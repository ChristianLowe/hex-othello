
mod board;
mod board_index;
mod config;
mod connector;
mod direction;
mod pieces;
mod strategy;

use crate::board::*;
use crate::board_index::*;
use crate::config::Config;
use crate::connector::ConnectorFactory;
use crate::strategy::StrategyFactory;

fn main() {
    let config = Config::from("config.toml");
    println!("Config: {:?}", config);

    let computer_player = Player::from_string(&config.run_params.ai_color);
    let mut connector = ConnectorFactory::from(&config);
    let strategy = StrategyFactory::from(&config);

    loop {
        let next_move_list = connector.get_next_move_list();
        let board = Board::from_move_list(&next_move_list);
        let current_player = if next_move_list.len() % 2 == 0 { Player::White } else { Player::Black };

        let moves = board.generate_moves(current_player);
        if moves.is_empty() {
            let other_moves = board.generate_moves(current_player.opposite());
            if other_moves.is_empty() {
                println!("Game over! White pieces: {}, Black pieces: {}",
                         board.get_piece_count(Player::White),
                         board.get_piece_count(Player::Black));
                break;
            }
        }

        if current_player == computer_player {
            println!("It's my turn!");
            let next_move = if moves.is_empty() {
                String::from("PASS")
            } else {
                let board_indexes = moves.board_indexes();
                let next_move = strategy.choose_move(board, board_indexes);
                BoardIndex::index_to_piece_name(next_move)
            };

            println!("Chose move {} from options {}", next_move, moves);
            println!("Board: {}", board);
            connector.submit_move(&next_move);
        }
    }
}
