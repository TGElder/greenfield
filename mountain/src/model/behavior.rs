use crate::network::skiing;

pub enum Behavior {
    _Stationary(skiing::State),
    Moving(Vec<Event>),
}

#[derive(Debug)]
pub struct Event {
    pub micros: u64,
    pub state: skiing::State,
}
