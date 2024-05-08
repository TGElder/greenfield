use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::model::skiing::State;

#[derive(Serialize, Deserialize)]
pub struct Exit {
    pub origin_piste_id: usize,
    pub stationary_states: HashSet<State>,
}
