use std::f32::consts::PI;

use commons::geometry::{xy, XY};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub const DIRECTIONS: [Direction; 8] = [
    Direction::East,
    Direction::NorthEast,
    Direction::North,
    Direction::NorthWest,
    Direction::West,
    Direction::SouthWest,
    Direction::South,
    Direction::SouthEast,
];

impl Direction {
    fn index(&self) -> usize {
        match self {
            Direction::East => 0,
            Direction::NorthEast => 1,
            Direction::North => 2,
            Direction::NorthWest => 3,
            Direction::West => 4,
            Direction::SouthWest => 5,
            Direction::South => 6,
            Direction::SouthEast => 7,
        }
    }

    pub fn angle(&self) -> f32 {
        (self.index() as f32 / 4.0) * PI
    }

    pub fn run(&self) -> f32 {
        match self {
            Direction::North | Direction::East | Direction::South | Direction::West => 1.0,
            Direction::SouthEast
            | Direction::SouthWest
            | Direction::NorthWest
            | Direction::NorthEast => 2.0f32.sqrt(),
        }
    }

    pub fn offset(&self) -> XY<i32> {
        match self {
            Direction::North => xy(0, 1),
            Direction::NorthEast => xy(1, 1),
            Direction::East => xy(1, 0),
            Direction::SouthEast => xy(1, -1),
            Direction::South => xy(0, -1),
            Direction::SouthWest => xy(-1, -1),
            Direction::West => xy(-1, 0),
            Direction::NorthWest => xy(-1, 1),
        }
    }

    pub fn next_anticlockwise(&self) -> Direction {
        let index = (self.index() + 1) % DIRECTIONS.len();
        DIRECTIONS[index]
    }

    pub fn next_clockwise(&self) -> Direction {
        if self.index() == 0 {
            return *DIRECTIONS.last().unwrap();
        }
        let index = self.index() - 1;
        DIRECTIONS[index]
    }
}
