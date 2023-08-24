use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::model::lift::Lift;
use crate::model::piste::Piste;
use crate::model::skiing::Plan;

pub fn run(
    plans: &HashMap<usize, Plan>,
    locations: &HashMap<usize, usize>,
    pistes: &HashMap<usize, Piste>,
    lifts: &HashMap<usize, Lift>,
    targets: &mut HashMap<usize, usize>,
) {
    for id in plans.keys() {
        if targets.contains_key(id) {
            continue;
        }
        let Some(location) = locations.get(id) else {
            continue;
        };
        let Some(piste) = pistes.get(location) else {
            continue;
        };

        let candidates = lifts
            .iter()
            .filter(|(_, lift)| piste.grid.in_bounds(lift.from))
            .filter(|(_, lift)| piste.grid[lift.from])
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        let choice = candidates.choose(&mut rand::thread_rng());

        if let Some(choice) = choice {
            targets.insert(*id, *choice);
        }
    }
}
