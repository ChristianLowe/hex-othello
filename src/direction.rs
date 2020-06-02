
use std::slice::Iter;

pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl Direction {
    pub fn mask(&self) -> u64 {
        match self {
            Direction::NorthWest    => 0xFEFEFEFEFEFEFE00,
            Direction::North        => 0xFFFFFFFFFFFFFFFF,
            Direction::NorthEast    => 0x7F7F7F7F7F7F7F00,
            Direction::East         => 0x7F7F7F7F7F7F7F7F,
            Direction::SouthEast    => 0x007F7F7F7F7F7F7F,
            Direction::South        => 0xFFFFFFFFFFFFFFFF,
            Direction::SouthWest    => 0x00FEFEFEFEFEFEFE,
            Direction::West         => 0xFEFEFEFEFEFEFEFE,
        }
    }

    pub fn iter() -> Iter<'static, Direction> {
        const DIRECTIONS: [Direction; 8] = [
            Direction::NorthWest,
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
        ];

        DIRECTIONS.iter()
    }
}
