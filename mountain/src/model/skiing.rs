use commons::geometry::XY;

use crate::model::Direction;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct State {
    pub position: XY<u32>,
    pub velocity: u8,
    pub travel_direction: Direction,
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
