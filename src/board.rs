
use std::fmt;
use std::fmt::Formatter;

use crate::board_index::*;
use crate::direction::Direction;
use crate::pieces::Pieces;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub(crate) fn opposite(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Board {
    white_pieces: Pieces,
    black_pieces: Pieces,
}

impl Board {
    pub fn new() -> Board { Board { white_pieces: Pieces::new(), black_pieces: Pieces::new() } }

    pub fn starter() -> Board {
        let mut board = Board::new();
        board.set_cell_state(BoardIndex::piece_name_to_index("D4"), Player::White);
        board.set_cell_state(BoardIndex::piece_name_to_index("E5"), Player::White);
        board.set_cell_state(BoardIndex::piece_name_to_index("E4"), Player::Black);
        board.set_cell_state(BoardIndex::piece_name_to_index("D5"), Player::Black);
        board
    }

    pub fn from_move_list(move_list: &Vec<String>) -> Board {
        let mut board = Board::starter();
        let mut player = Player::White;
        for player_move in move_list.iter() {
            if !player_move.eq("pass") {
                let index = BoardIndex::piece_name_to_index(player_move.as_str());
                board = board.resolve_move(player, index);
            }
            player = player.opposite();
        }
        board
    }

    // Generate move board using Dumb7Fill algorithm
    pub fn generate_moves(&self, player: Player) -> Pieces {
        let players = self.get_ordered_players(player);
        let friend_pieces = self.get_player_pieces(players.0);
        let enemy_pieces = self.get_player_pieces(players.1);
        let empty_pieces = !(friend_pieces | enemy_pieces);

        let mut legal_moves = Pieces::new();
        for direction in Direction::iter() {
            let mut direction_moves = friend_pieces.with_slide(direction) & enemy_pieces;
            for _ in 0..5 {
                direction_moves |= direction_moves.with_slide(direction) & enemy_pieces;
            }
            legal_moves |= direction_moves.with_slide(direction) & empty_pieces;
        }

        legal_moves
    }

    pub fn resolve_move(&self, player: Player, index: BoardIndex) -> Board {
        let players = self.get_ordered_players(player);
        let mut friend_pieces = self.get_player_pieces(players.0);
        let mut enemy_pieces = self.get_player_pieces(players.1);
        debug_assert!((friend_pieces & enemy_pieces).is_empty(), "Board piece sets must be disjoint.");

        let new_piece: Pieces = Pieces::from_index(index);
        debug_assert!(((friend_pieces | enemy_pieces) & new_piece).is_empty(), "Target must be empty.");

        friend_pieces |= new_piece;

        let mut captured_pieces = Pieces::new();
        for direction in Direction::iter() {
            let mut direction_moves = new_piece.with_slide(direction) & enemy_pieces;
            for _ in 0..5 {
                direction_moves |= direction_moves.with_slide(direction) & enemy_pieces;
            }

            let bounding_piece = direction_moves.with_slide(direction) & friend_pieces;
            if !bounding_piece.is_empty() {
                captured_pieces |= direction_moves;
            }
        }
        debug_assert!(captured_pieces.is_not_empty(), "A valid move must capture disks.");

        friend_pieces ^= captured_pieces;
        enemy_pieces ^= captured_pieces;
        debug_assert!((friend_pieces & enemy_pieces).is_empty(), "Board piece sets must still be disjoint.");

        match player {
            Player::White => Board { white_pieces: friend_pieces, black_pieces: enemy_pieces },
            Player::Black => Board { black_pieces: friend_pieces, white_pieces: enemy_pieces },
        }
    }

    pub fn get_piece_count(&self, player: Player) -> u32 {
        match player {
            Player::White => self.white_pieces.piece_count(),
            Player::Black => self.black_pieces.piece_count(),
        }
    }

    fn get_ordered_players(&self, player: Player) -> (Player, Player) {
        match player {
            Player::White => (Player::White, Player::Black),
            Player::Black => (Player::Black, Player::White),
        }
    }

    fn get_player_pieces(&self, player: Player) -> Pieces {
        match player {
            Player::White => self.white_pieces,
            Player::Black => self.black_pieces,
        }
    }

    fn set_cell_state(&mut self, index: BoardIndex, player: Player) {
        let mask = Pieces::from_index(index);

        match player {
            Player::White => {
                self.black_pieces &= !mask;
                self.white_pieces |= mask;
            },
            Player::Black => {
                self.white_pieces &= !mask;
                self.black_pieces |= mask;
            },
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Board {{ white_pieces: {}; black_pieces: {} }}", self.white_pieces, self.black_pieces)
    }
}
