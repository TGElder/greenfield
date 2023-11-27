use std::collections::{HashMap, HashSet};

use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::Lift;
use crate::model::piste::Piste;

pub fn run(
    pistes: &HashMap<usize, Piste>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
    exits: &mut HashMap<usize, Vec<Exit>>,
) {
    exits.clear();
    for (piste_id, piste) in pistes.iter() {
        let exits_for_piste = exits_for_piste(piste_id, piste, lifts, entrances);
        exits.insert(*piste_id, exits_for_piste);
    }
}

fn exits_for_piste(
    piste_id: &usize,
    piste: &Piste,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
) -> Vec<Exit> {
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
        .map(|(id, positions)| Exit { id: *id, positions })
        .collect()
}
