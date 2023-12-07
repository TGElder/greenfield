use std::collections::HashMap;

use commons::origin_grid::OriginGrid;
use serde::{Deserialize, Serialize};

use crate::model::skiing::State;

#[derive(Serialize, Deserialize)]
pub struct Piste {
    pub grid: OriginGrid<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PisteCosts {
    lift_to_costs: HashMap<usize, HashMap<State, u64>>,
}

impl PisteCosts {
    pub fn new() -> PisteCosts {
        PisteCosts {
            lift_to_costs: HashMap::new(),
        }
    }

    pub fn costs(&self, lift: &usize) -> Option<&HashMap<State, u64>> {
        self.lift_to_costs.get(lift)
    }

    pub fn set_costs(&mut self, lift: usize, costs: HashMap<State, u64>) {
        self.lift_to_costs.insert(lift, costs);
    }
}
