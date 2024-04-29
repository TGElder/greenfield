use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::model::skiing::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entrance {
    pub id: usize,
    pub stationary_states: HashSet<State>,
}
