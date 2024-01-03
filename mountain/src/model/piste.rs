use std::collections::{HashMap, HashSet};

use commons::origin_grid::OriginGrid;
use serde::{Deserialize, Serialize};

use crate::model::skiing::State;

#[derive(Serialize, Deserialize)]
pub struct Piste {
    pub grid: OriginGrid<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Costs {
    target_to_costs: HashMap<usize, HashMap<State, u64>>,
}

impl Costs {
    pub fn new() -> Costs {
        Costs {
            target_to_costs: HashMap::new(),
        }
    }

    pub fn costs(&self, target: &usize) -> Option<&HashMap<State, u64>> {
        self.target_to_costs.get(target)
    }

    pub fn set_costs(&mut self, target: usize, costs: HashMap<State, u64>) {
        self.target_to_costs.insert(target, costs);
    }

    pub fn remove_costs(&mut self, target: &usize) {
        self.target_to_costs.remove(target);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Basins {
    target_to_basin: HashMap<usize, HashSet<State>>,
}

impl Basins {
    pub fn new() -> Basins {
        Basins {
            target_to_basin: HashMap::new(),
        }
    }

    pub fn set_basin(&mut self, target: usize, basin: HashSet<State>) {
        self.target_to_basin.insert(target, basin);
    }
}
