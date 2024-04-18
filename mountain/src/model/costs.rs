use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::ability::Ability;
use crate::model::skiing::State;

#[derive(Eq, Hash, PartialEq, Serialize, Deserialize)]
struct Key {
    target: usize,
    ability: Ability,
}

#[derive(Serialize, Deserialize)]
pub struct Costs {
    key_to_costs: HashMap<Key, HashMap<State, u64>>,
}

impl Costs {
    pub fn new() -> Costs {
        Costs {
            key_to_costs: HashMap::new(),
        }
    }

    pub fn costs(&self, target: usize, ability: Ability) -> Option<&HashMap<State, u64>> {
        self.key_to_costs.get(&Key { target, ability })
    }

    pub fn set_costs(&mut self, target: usize, ability: Ability, costs: HashMap<State, u64>) {
        self.key_to_costs.insert(Key { target, ability }, costs);
    }

    pub fn remove_costs(&mut self, target: usize, ability: Ability) {
        self.key_to_costs.remove(&Key { target, ability });
    }

    pub fn targets_reachable_from_state<'a>(
        &'a self,
        state: &'a State,
        ability: &'a Ability,
    ) -> impl Iterator<Item = (&usize, &u64)> + 'a {
        self.key_to_costs
            .iter()
            .filter(|(key, _)| key.ability == *ability)
            .flat_map(move |(Key { target, .. }, costs)| {
                costs.get(state).map(|cost| (target, cost))
            })
    }

    pub fn min_ability(&self, from: &State, target: &usize) -> Option<Ability> {
        self.key_to_costs
            .iter()
            .filter(|(key, _)| key.target == *target)
            .filter(|(_, costs)| costs.contains_key(from))
            .map(|(key, _)| key.ability)
            .min()
    }
}
