use commons::geometry::XY;
use serde::{Deserialize, Serialize};

use crate::model::direction::Direction;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub position: XY<u32>,
    pub mode: Mode,
    pub travel_direction: Direction,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    Walking,
    Skiing { velocity: u8 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Plan {
    Stationary(State),
    Moving(Vec<Event>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub micros: u128,
    pub state: State,
}
