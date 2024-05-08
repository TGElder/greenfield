use std::collections::{HashMap, HashSet};

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
    exits: &mut HashMap<usize, Exit>,
) {
    let Some(piste) = pistes.get(piste_id) else {
        return;
    };

    let piste_exits = compute_piste_exits(piste_id, piste, lifts, gates);

    exits.extend(piste_exits);
}

fn compute_piste_exits(
    piste_id: &usize,
    piste: &Piste,
    lifts: &HashMap<usize, Lift>,
    gates: &HashMap<usize, Gate>,
) -> HashMap<usize, Exit> {
    let piste = &piste.grid;

    let lifts_iter = lifts
        .iter()
        .filter(|(_, lift)| {
            let position = lift.pick_up.state.position;
            piste.in_bounds(position) && piste[position]
        })
        .map(|(&id, lift)| {
            (
                id,
                Exit {
                    origin_piste_id: *piste_id,
                    stationary_states: HashSet::from([lift.pick_up.state.stationary()]),
                },
            )
        });

    let gates_iter = gates
        .iter()
        .filter(|(_, gate)| gate.origin_piste == *piste_id)
        .map(|(&id, gate)| {
            (
                id,
                Exit {
                    origin_piste_id: *piste_id,
                    stationary_states: gate
                        .footprint
                        .iter()
                        .filter(|position| piste.in_bounds(position) && piste[position])
                        .flat_map(|position| {
                            DIRECTIONS.iter().map(move |&travel_direction| State {
                                position,
                                velocity: 0,
                                travel_direction,
                            })
                        })
                        .collect::<HashSet<_>>(),
                },
            )
        });

    lifts_iter.chain(gates_iter).collect::<HashMap<_, _>>()
}
