use std::collections::HashMap;

use commons::origin_grid::OriginGrid;
use serde::{Deserialize, Serialize};

use crate::model::ability::Ability;
use crate::model::skiing::State;

#[derive(Serialize, Deserialize)]
pub struct Piste {
    pub grid: OriginGrid<bool>,
}

#[derive(Eq, Hash, PartialEq, Serialize, Deserialize)]
struct Key {
    target: usize,
    ability: Ability,
}

#[derive(Serialize, Deserialize)]
pub struct Costs {
    target_to_costs: HashMap<Key, HashMap<State, u64>>,
}

impl Costs {
    pub fn new() -> Costs {
        Costs {
            target_to_costs: HashMap::new(),
        }
    }

    pub fn costs(&self, target: usize, ability: Ability) -> Option<&HashMap<State, u64>> {
        self.target_to_costs.get(&Key { target, ability })
    }

    pub fn set_costs(&mut self, target: usize, ability: Ability, costs: HashMap<State, u64>) {
        self.target_to_costs.insert(Key { target, ability }, costs);
    }

    pub fn remove_costs(&mut self, target: usize, ability: Ability) {
        self.target_to_costs.remove(&Key { target, ability });
    }

    pub fn targets_reachable_from_state<'a>(
        &'a self,
        state: &'a State,
        ability: &'a Ability,
    ) -> impl Iterator<Item = &usize> + 'a {
        self.target_to_costs
            .iter()
            .filter(|(key, _)| key.ability == *ability)
            .filter(move |(_, costs)| costs.contains_key(state))
            .map(|(Key { target, .. }, _)| target)
    }
}
