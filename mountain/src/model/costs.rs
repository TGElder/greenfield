use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::model::ability::Ability;

#[derive(Eq, Hash, PartialEq, Serialize, Deserialize)]
struct Key {
    target: usize,
    ability: Ability,
}

#[derive(Serialize, Deserialize)]
pub struct Costs<T>
where
    T: Eq + Hash,
{
    key_to_costs: HashMap<Key, HashMap<T, u64>>,
}

impl<T> Costs<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Costs<T> {
        Costs {
            key_to_costs: HashMap::new(),
        }
    }

    pub fn costs(&self, target: usize, ability: Ability) -> Option<&HashMap<T, u64>> {
        self.key_to_costs.get(&Key { target, ability })
    }

    pub fn set_costs(&mut self, target: usize, ability: Ability, costs: HashMap<T, u64>) {
        self.key_to_costs.insert(Key { target, ability }, costs);
    }

    pub fn remove_costs(&mut self, target: usize, ability: Ability) {
        self.key_to_costs.remove(&Key { target, ability });
    }

    pub fn targets_reachable_from_node<'a>(
        &'a self,
        node: &'a T,
        ability: &'a Ability,
    ) -> impl Iterator<Item = (&'a usize, &'a u64)> + 'a {
        self.key_to_costs
            .iter()
            .filter(|(key, _)| key.ability == *ability)
            .flat_map(move |(Key { target, .. }, costs)| costs.get(node).map(|cost| (target, cost)))
    }

    pub fn min_ability(&self, from: &T, target: &usize) -> Option<Ability> {
        self.key_to_costs
            .iter()
            .filter(|(key, _)| key.target == *target)
            .filter(|(_, costs)| costs.contains_key(from))
            .map(|(key, _)| key.ability)
            .min()
    }
}
