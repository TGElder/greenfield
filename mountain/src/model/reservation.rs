use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]

pub enum Reservation {
    #[default]
    None,
    Until(u128),
    Eternal,
}

impl Reservation {
    pub fn is_reserved(&self, micros: &u128) -> bool {
        match self {
            Reservation::None => false,
            Reservation::Until(until) => micros <= until,
            Reservation::Eternal => true,
        }
    }
}
