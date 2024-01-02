use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]

pub enum Reservation {
    #[default]
    None,
    Until(u128),
    Permanent,
}

impl Reservation {
    pub fn is_valid_at(&self, micros: &u128) -> bool {
        match self {
            Reservation::None => false,
            Reservation::Until(until) => micros <= until,
            Reservation::Permanent => true,
        }
    }
}
