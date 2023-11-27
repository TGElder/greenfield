use std::collections::{HashMap, HashSet};

use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::piste::Piste;
use crate::model::target::Target;

pub fn run(
    pistes: &HashMap<usize, Piste>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
    valid_targets: &mut HashMap<usize, Vec<Target>>,
) {
    valid_targets.clear();
    for (piste_id, piste) in pistes.iter() {
        let targets = valid_targets_for_piste(piste_id, piste, lifts, entrances);
        valid_targets.insert(*piste_id, targets);
    }
}

fn valid_targets_for_piste(
    piste_id: &usize,
    piste: &Piste,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
) -> Vec<Target> {
    let grid = &piste.grid;

    let lifts_iter = lifts
        .iter()
        .map(|(lift_id, lift)| (lift_id, vec![lift.pick_up.position]));

    let entrances_iter = entrances
        .iter()
        .filter(|(_, entrance)| entrance.piste != *piste_id)
        .map(|(entrance_id, entrance)| {
            (
                entrance_id,
                entrance
                    .footprint
                    .iter()
                    .filter(|position| grid.in_bounds(position))
                    .collect::<Vec<_>>(),
            )
        });

    lifts_iter
        .chain(entrances_iter)
        .map(|(id, positions)| {
            (
                id,
                positions
                    .into_iter()
                    .filter(|position| grid.in_bounds(position) && grid[position])
                    .collect::<HashSet<_>>(),
            )
        })
        .filter(|(_, positions)| !positions.is_empty())
        .map(|(id, positions)| Target { id: *id, positions })
        .collect()
}
