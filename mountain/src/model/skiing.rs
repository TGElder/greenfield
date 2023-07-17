use commons::geometry::XY;

use crate::model::direction::Direction;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct State {
    pub position: XY<u32>,
    pub mode: Mode,
    pub travel_direction: Direction,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Mode {
    Walking,
    Skiing { velocity: u8 },
}

#[derive(Debug)]
pub enum Plan {
    Stationary(State),
    Moving(Vec<Event>),
}

#[derive(Debug)]
pub struct Event {
    pub micros: u128,
    pub state: State,
}
