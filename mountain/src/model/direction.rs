use std::f32::consts::PI;

use commons::geometry::{xy, XY};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
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

    pub fn offset(&self) -> XY<i32> {
        match self {
            Direction::East => xy(1, 0),
            Direction::NorthEast => xy(1, 1),
            Direction::North => xy(0, 1),
            Direction::NorthWest => xy(-1, 1),
            Direction::West => xy(-1, 0),
            Direction::SouthWest => xy(-1, -1),
            Direction::South => xy(0, -1),
            Direction::SouthEast => xy(1, -1),
        }
    }

    pub fn run(&self) -> f32 {
        match self {
            Direction::East | Direction::North | Direction::West | Direction::South => 1.0,
            Direction::NorthEast
            | Direction::NorthWest
            | Direction::SouthWest
            | Direction::SouthEast => 2.0f32.sqrt(),
        }
    }

    pub fn next_clockwise(&self) -> Direction {
        if self.index() == 0 {
            return *DIRECTIONS.last().unwrap();
        }
        let index = self.index() - 1;
        DIRECTIONS[index]
    }

    pub fn next_anticlockwise(&self) -> Direction {
        let index = (self.index() + 1) % DIRECTIONS.len();
        DIRECTIONS[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_clockwise() {
        assert_eq!(Direction::East.next_clockwise(), Direction::SouthEast);
        assert_eq!(Direction::SouthEast.next_clockwise(), Direction::South);
        assert_eq!(Direction::South.next_clockwise(), Direction::SouthWest);
        assert_eq!(Direction::SouthWest.next_clockwise(), Direction::West);
        assert_eq!(Direction::West.next_clockwise(), Direction::NorthWest);
        assert_eq!(Direction::NorthWest.next_clockwise(), Direction::North);
        assert_eq!(Direction::North.next_clockwise(), Direction::NorthEast);
        assert_eq!(Direction::NorthEast.next_clockwise(), Direction::East);
    }

    #[test]
    fn test_next_anticlockwise() {
        assert_eq!(Direction::East.next_anticlockwise(), Direction::NorthEast);
        assert_eq!(Direction::NorthEast.next_anticlockwise(), Direction::North);
        assert_eq!(Direction::North.next_anticlockwise(), Direction::NorthWest);
        assert_eq!(Direction::NorthWest.next_anticlockwise(), Direction::West);
        assert_eq!(Direction::West.next_anticlockwise(), Direction::SouthWest);
        assert_eq!(Direction::SouthWest.next_anticlockwise(), Direction::South);
        assert_eq!(Direction::South.next_anticlockwise(), Direction::SouthEast);
        assert_eq!(Direction::SouthEast.next_anticlockwise(), Direction::East);
    }
}
