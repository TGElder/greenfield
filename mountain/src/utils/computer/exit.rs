use std::collections::{HashMap, HashSet};

use commons::geometry::XY;

use crate::model::direction::DIRECTIONS;
use crate::model::exit::Exit;
use crate::model::gate::Gate;
use crate::model::lift::Lift;
use crate::model::piste::Piste;
use crate::model::skiing::State;

pub fn compute_piste(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    lifts: &HashMap<usize, Lift>,
    gates: &HashMap<usize, Gate>,
    exits: &mut HashMap<usize, Vec<Exit>>,
) {
    exits.remove(piste_id);

    let Some(piste) = pistes.get(piste_id) else {
        return;
    };

    let exits_for_piste = exits_for_piste(piste_id, piste, lifts, gates);

    exits.insert(*piste_id, exits_for_piste);
}

fn exits_for_piste(
    piste_id: &usize,
    piste: &Piste,
    lifts: &HashMap<usize, Lift>,
    gates: &HashMap<usize, Gate>,
) -> Vec<Exit> {
    let grid = &piste.grid;

    let lifts_iter = lifts.iter().flat_map(|(lift_id, lift)| {
        Some(Exit {
            id: *lift_id,
            states: HashSet::from([lift.pick_up.state]),
        })
    });

    let gates_iter = gates
        .iter()
        .filter(|(_, gate)| gate.destination_piste != *piste_id)
        .map(|(gate_id, gate)| Exit {
            id: *gate_id,
            states: gate
                .footprint
                .iter()
                .filter(|position| grid.in_bounds(position))
                .flat_map(stationary_states_for_position)
                .collect::<HashSet<_>>(),
        });

    lifts_iter
        .chain(gates_iter)
        .map(|exit| Exit {
            states: exit
                .states
                .into_iter()
                .filter(|State { position, .. }| grid.in_bounds(position) && grid[position])
                .collect::<HashSet<_>>(),
            ..exit
        })
        .filter(
            |Exit {
                 states: positions, ..
             }| !positions.is_empty(),
        )
        .collect()
}

fn stationary_states_for_position(position: XY<u32>) -> impl Iterator<Item = State> {
    DIRECTIONS.into_iter().map(move |travel_direction| State {
        position,
        velocity: 0,
        travel_direction,
    })
}
