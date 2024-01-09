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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Ability {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl Ability {
    fn index(&self) -> u8 {
        match self {
            Ability::Beginner => 0,
            Ability::Intermediate => 1,
            Ability::Advanced => 2,
            Ability::Expert => 3,
        }
    }
}

impl PartialOrd for Ability {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index().partial_cmp(&other.index())
    }
}

pub const ABILITIES: [Ability; 4] = [
    Ability::Beginner,
    Ability::Intermediate,
    Ability::Advanced,
    Ability::Expert,
];
