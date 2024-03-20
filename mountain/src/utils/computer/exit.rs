use std::collections::{HashMap, HashSet};

use commons::grid::Grid;

use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::Lift;
use crate::model::piste::Piste;

pub fn compute_piste(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    piste_map: &Grid<Option<usize>>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
    exits: &mut HashMap<usize, Vec<Exit>>,
) {
    exits.remove(piste_id);

    let Some(piste) = pistes.get(piste_id) else {
        return;
    };

    let exits_for_piste = exits_for_piste(piste_id, piste, piste_map, lifts, entrances);

    exits.insert(*piste_id, exits_for_piste);
}

fn exits_for_piste(
    piste_id: &usize,
    piste: &Piste,
    piste_map: &Grid<Option<usize>>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
) -> Vec<Exit> {
    let grid = &piste.grid;

    let lifts_iter = lifts.iter().flat_map(|(lift_id, lift)| {
        Some(Exit {
            id: *lift_id,
            destination: piste_map[lift.drop_off.position]?,
            positions: HashSet::from([lift.pick_up.position]),
        })
    });

    let entrances_iter = entrances
        .iter()
        .filter(|(_, entrance)| entrance.piste != *piste_id)
        .map(|(entrance_id, entrance)| Exit {
            id: *entrance_id,
            destination: entrance.piste,
            positions: entrance
                .footprint
                .iter()
                .filter(|position| grid.in_bounds(position))
                .collect::<HashSet<_>>(),
        });

    lifts_iter
        .chain(entrances_iter)
        .map(|exit| Exit {
            positions: exit
                .positions
                .into_iter()
                .filter(|position| grid.in_bounds(position) && grid[position])
                .collect::<HashSet<_>>(),
            ..exit
        })
        .filter(|Exit { positions, .. }| !positions.is_empty())
        .collect()
}
