use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::model::skiing::State;

#[derive(Serialize, Deserialize)]
pub struct Exit {
    pub id: usize,
    pub destination: usize,
    pub states: HashSet<State>,
}
