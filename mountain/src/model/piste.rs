use std::collections::HashMap;

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

    pub fn targets_reachable_from_state<'a>(
        &'a self,
        state: &'a State,
    ) -> impl Iterator<Item = &usize> + 'a {
        self.target_to_costs
            .iter()
            .filter(move |(_, costs)| costs.contains_key(state))
            .map(|(target, _)| target)
    }
}
