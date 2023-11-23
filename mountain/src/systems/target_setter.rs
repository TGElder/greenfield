use std::collections::HashMap;

use commons::geometry::xy;
use commons::grid::Grid;
use rand::seq::SliceRandom;

use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::piste::Piste;
use crate::model::skiing::Plan;

pub fn run(
    terrain: &Grid<f32>,
    plans: &HashMap<usize, Plan>,
    locations: &HashMap<usize, usize>,
    pistes: &HashMap<usize, Piste>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
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

        let grid = &piste.grid;

        let lifts_iter = lifts
            .iter()
            .map(|(lift_id, lift)| (lift_id, vec![lift.pick_up.position]));

        let entrances_iter = entrances
            .iter()
            .filter(|(_, entrance)| entrance.piste != *location)
            .map(|(entrance_id, entrance)| {
                (
                    entrance_id,
                    (entrance.from.x..=entrance.to.x)
                        .flat_map(|x| (entrance.from.y..=entrance.to.y).map(move |y| xy(x, y)))
                        .filter(|position| grid.in_bounds(position))
                        .collect::<Vec<_>>(),
                )
            });

        let candidates = lifts_iter
            .chain(entrances_iter)
            .map(|(id, positions)| {
                (
                    id,
                    positions
                        .into_iter()
                        .filter(|position| grid.in_bounds(position) && grid[position])
                        .filter(|position| terrain[position] < elevation)
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(_, positions)| !positions.is_empty())
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();

        let choice = candidates.choose(&mut rand::thread_rng());

        if let Some(choice) = choice {
            targets.insert(*id, *choice);
        }
    }
}
