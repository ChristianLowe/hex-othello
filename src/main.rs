
mod board;
mod board_index;
mod direction;
mod pieces;
mod web_client;

use crate::board::*;
use crate::board_index::*;
use crate::web_client::{WebClient, WebGame};

use rand::seq::SliceRandom;
use reqwest::Error;

use std::{io, thread, time};
use std::io::*;

fn main() {
    let poll_wait_time = time::Duration::from_millis(1500);

    let mut game_id = String::from("");
    if game_id.is_empty() {
        println!("Enter game id: ");
        io::stdin().read_line(&mut game_id).unwrap();
    }

    let computer_player = Player::Black;

    let web_client = WebClient::from(game_id);
    loop {
        let web_game = web_client.get_latest_web_game();
        if web_game.is_err() {
            let err = web_game.unwrap_err();
            println!("Error making web call: {}", err);
            thread::sleep(poll_wait_time);
            continue;
        }

        let web_game = web_game.unwrap();
        let board = Board::from_move_list(&web_game.moves);
        let current_player = if web_game.moves.len() % 2 == 0 { Player::White } else { Player::Black };

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
            let player_move = if moves.is_empty() {
                String::from("PASS")
            } else {
                let board_indexes = moves.board_indexes();
                let random_move = board_indexes.choose(&mut rand::thread_rng()).unwrap();
                BoardIndex::index_to_piece_name(*random_move)
            };

            println!("Chose move {} from options {}", player_move, moves);
            let response = web_client.submit_move(&web_game, player_move);
            println!("Response: {:?}", response);
        } else {
            println!("Not my turn :(");
        }

        thread::sleep(poll_wait_time);
    }

    //
    // println!("Game: {:?}", web_game);
    //
    // match web_game {
    //     Ok(result) => {
    //         let board = Board::from_move_list(result.moves);
    //         println!("Board: {}", board);
    //     },
    //     Err(err) => {
    //         println!("Error: {}", err);
    //     },
    // }


    // let mut board = Board::starter();
    // println!("Starter board: {}", board);
    //
    // let mut player = Player::White;
    // loop {
    //     let moves = board.generate_moves(player);
    //     if moves.is_empty() {
    //         let other_moves = board.generate_moves(player.opposite());
    //         if other_moves.is_not_empty() {
    //             player = player.opposite();
    //             continue;
    //         }
    //
    //         let white_count = board.get_piece_count(Player::White);
    //         let black_count = board.get_piece_count(Player::Black);
    //         println!("Game over! White count: {}, Black count: {}", white_count, black_count);
    //         break;
    //     }
    //
    //     let board_indexes = moves.board_indexes();
    //     let board_index = board_indexes.choose(&mut rand::thread_rng()).unwrap();
    //
    //     println!("{:?} options [{}]; move: {}", player, moves, BoardIndex::index_to_piece_name(*board_index));
    //     board = board.resolve_move(player, *board_index);
    //     println!("New board: {}", board);
    //
    //     player = player.opposite();
    // }

}
