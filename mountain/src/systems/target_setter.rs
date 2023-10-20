use std::collections::{HashMap, HashSet};

use commons::grid::Grid;
use rand::seq::SliceRandom;

use crate::model::lift::{self, Lift};
use crate::model::piste::Piste;
use crate::model::skiing::Plan;

pub fn run(
    terrain: &Grid<f32>,
    plans: &HashMap<usize, Plan>,
    locations: &HashMap<usize, usize>,
    pistes: &HashMap<usize, Piste>,
    lifts: &HashMap<usize, Lift>,
    targets: &mut HashMap<usize, usize>,
) {
    for (id, plan) in plans {
        let Plan::Stationary(state) = plan else {
            continue;
        };
        if targets.contains_key(id) {
            continue;
        }
        let Some(location) = locations.get(id) else {
            continue;
        };
        let Some(piste) = pistes.get(location) else {
            continue;
        };

        let elevation = terrain[state.position];

        let candidates = lifts
            .iter()
            .flat_map(|(i, lift)| lift.nodes.iter().map(move |node| (i, node)))
            .flat_map(|(i, node)| match node.from_action {
                Some(lift::Action::PickUp(position)) => Some((i, position)),
                _ => None,
            })
            .filter(|(_, position)| piste.grid.in_bounds(position))
            .filter(|(_, position)| piste.grid[position])
            .filter(|(_, position)| terrain[position] < elevation)
            .map(|(id, _)| *id)
            .collect::<HashSet<_>>();
        let candidaes = candidates.into_iter().collect::<Vec<_>>();
        let choice = candidaes.choose(&mut rand::thread_rng());

        if let Some(choice) = choice {
            targets.insert(*id, *choice);
        }
    }
}
