use std::collections::HashMap;

use commons::grid::Grid;
use rand::seq::SliceRandom;

use crate::model::exit::Exit;
use crate::model::skiing::Plan;

pub fn run(
    terrain: &Grid<f32>,
    plans: &HashMap<usize, Plan>,
    locations: &HashMap<usize, usize>,
    exits: &HashMap<usize, Vec<Exit>>,
    targets: &mut HashMap<usize, usize>,
) {
    for (plan_id, plan) in plans {
        let Plan::Stationary(state) = plan else {
            continue;
        };
        if targets.contains_key(plan_id) {
            continue;
        }
        let Some(location_id) = locations.get(plan_id) else {
            continue;
        };

        let elevation = terrain[state.position];

        let candidates = exits
            .get(location_id)
            .into_iter()
            .flatten()
            .filter(|Exit { positions, .. }| {
                positions
                    .iter()
                    .any(|position| terrain[position] < elevation)
            })
            .map(|Exit { id, .. }| *id)
            .collect::<Vec<_>>();
        let choice = candidates.choose(&mut rand::thread_rng());

        if let Some(choice) = choice {
            targets.insert(*plan_id, *choice);
        }
    }
}
