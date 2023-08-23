use std::collections::HashMap;

use commons::grid::Grid;

use crate::model::skiing::{Plan, State};

pub fn run(
    plans: &HashMap<usize, Plan>,
    piste_map: &Grid<Option<usize>>,
    locations: &mut HashMap<usize, usize>,
) {
    for (id, plan) in plans.iter() {
        if locations.contains_key(id) {
            continue;
        }
        let Plan::Stationary(State{position, ..}) = plan else {continue};

        if let Some(piste) = piste_map[position] {
            locations.insert(*id, piste);
        }
    }
}
