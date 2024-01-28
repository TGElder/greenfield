use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub enum Reservation {
    Structure,
    Mobile(ReservationPeriod),
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub enum ReservationPeriod {
    Until(u128),
    Permanent,
}

impl Reservation {
    pub fn includes(&self, micros: &u128) -> bool {
        match self {
            Reservation::Structure => true,
            Reservation::Mobile(ReservationPeriod::Until(until)) => micros <= until,
            Reservation::Mobile(ReservationPeriod::Permanent) => true,
        }
    }
}
