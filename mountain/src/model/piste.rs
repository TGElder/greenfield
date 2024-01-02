use std::collections::{HashMap, HashSet};

use commons::origin_grid::OriginGrid;
use serde::{Deserialize, Serialize};

use crate::model::skiing::State;

#[derive(Serialize, Deserialize)]
pub struct Piste {
    pub grid: OriginGrid<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PisteCosts {
    target_to_costs: HashMap<usize, HashMap<State, u64>>,
}

impl PisteCosts {
    pub fn new() -> PisteCosts {
        PisteCosts {
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
pub struct Reachability {
    target_to_reachable: HashMap<usize, HashSet<State>>,
}

pub fn new() -> PisteCosts {
    PisteCosts {
        target_to_costs: HashMap::new(),
    }
}

impl Reachability {
    pub fn set_reachable(&mut self, target: usize, reachable: HashSet<State>) {
        self.target_to_reachable.insert(target, reachable);
    }
}
