
pub type BoardIndex = u8;

pub trait _BoardIndex {
    fn piece_name_to_index(piece_name: &str) -> BoardIndex;
    fn index_to_piece_name(index: BoardIndex) -> String;
}

impl _BoardIndex for BoardIndex {
    fn piece_name_to_index(piece_name: &str) -> BoardIndex {
        debug_assert!(piece_name.len() == 2, "Piece name should be two characters");
        debug_assert!(piece_name.is_ascii(), "Piece name should contain only ASCII characters");

        let piece_bytes = piece_name.as_bytes();
        debug_assert!(piece_bytes[0] >= b'A' && piece_bytes[0] <= b'H', "First piece name byte should be a character from A-H");
        debug_assert!(piece_bytes[1] >= b'1' && piece_bytes[1] <= b'8', "Second piece name byte should be a number from 1-8");

        let index = piece_bytes[0] - b'A';
        let index = index + (piece_bytes[1] - b'1') * 8;
        index as BoardIndex
    }

    fn index_to_piece_name(index: BoardIndex) -> String {
        debug_assert!(index < 64, "Board index should be between 0-64");

        let col = b'A' + (index % 8) as u8;
        let row = b'1' + (index / 8) as u8;
        unsafe { String::from_utf8_unchecked(vec!(col, row)) }
    }
}


