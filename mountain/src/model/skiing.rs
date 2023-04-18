use commons::geometry::XY;

use crate::model::Direction;

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub position: XY<u32>,
    pub velocity: u8,
    pub travel_direction: Direction,
}

pub enum Plan {
    _Stationary(State),
    Moving(Vec<Event>),
}

#[derive(Debug)]
pub struct Event {
    pub micros: u64,
    pub state: State,
}
