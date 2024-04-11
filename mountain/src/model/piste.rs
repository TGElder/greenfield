use std::collections::HashMap;

use commons::origin_grid::OriginGrid;
use serde::{Deserialize, Serialize};

use crate::model::ability::Ability;
use std::hash::Hash;

#[derive(Serialize, Deserialize)]
pub struct Piste {
    pub grid: OriginGrid<bool>,
}

#[derive(Eq, Hash, PartialEq, Serialize, Deserialize)]
struct Key {
    target: usize,
    ability: Ability,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Costs<T>
// TODO move out of piste mod
where
    T: Eq + Hash,
{
    target_to_costs: HashMap<Key, HashMap<T, u64>>,
}

impl<T> Costs<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Costs<T> {
        Costs {
            target_to_costs: HashMap::new(),
        }
    }

    pub fn costs(&self, target: usize, ability: Ability) -> Option<&HashMap<T, u64>> {
        self.target_to_costs.get(&Key { target, ability })
    }

    pub fn set_costs(&mut self, target: usize, ability: Ability, costs: HashMap<T, u64>) {
        self.target_to_costs.insert(Key { target, ability }, costs);
    }

    pub fn remove_costs(&mut self, target: usize, ability: Ability) {
        self.target_to_costs.remove(&Key { target, ability });
    }

    pub fn targets_reachable_from_state<'a>(
        &'a self,
        state: &'a T,
        ability: &'a Ability,
    ) -> impl Iterator<Item = &usize> + 'a {
        self.target_to_costs
            .iter()
            .filter(|(key, _)| key.ability == *ability)
            .filter(move |(_, costs)| costs.contains_key(state))
            .map(|(Key { target, .. }, _)| target)
    }

    pub fn min_ability(&self, from: &T, target: &usize) -> Option<Ability> {
        self.target_to_costs
            .iter()
            .filter(|(key, _)| key.target == *target)
            .filter(|(_, costs)| costs.contains_key(from))
            .map(|(key, _)| key.ability)
            .min()
    }
}
