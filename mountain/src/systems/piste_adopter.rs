use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::model::piste::Piste;
use crate::model::skiing::{Plan, State};

pub fn run(
    plans: &HashMap<usize, Plan>,
    pistes: &HashMap<usize, Piste>,
    locations: &mut HashMap<usize, usize>,
) {
    for (id, plan) in plans.iter() {
        if locations.contains_key(id) {
            continue;
        }
        let Plan::Stationary(State{position, ..}) = plan else {continue};

        let candidates = pistes
            .iter()
            .filter(|(_, piste)| piste.grid.in_bounds(position))
            .filter(|(_, piste)| piste.grid[position])
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        let choice = candidates.choose(&mut rand::thread_rng());

        if let Some(choice) = choice {
            locations.insert(*id, *choice);
        }
    }
}
