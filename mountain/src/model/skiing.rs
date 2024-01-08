use commons::geometry::XY;
use serde::{Deserialize, Serialize};

use crate::model::direction::Direction;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub position: XY<u32>,
    pub velocity: u8,
    pub travel_direction: Direction,
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
