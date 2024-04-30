use std::collections::{HashMap, HashSet};

use crate::model::direction::DIRECTIONS;
use crate::model::entrance::Entrance;
use crate::model::gate::Gate;
use crate::model::lift::Lift;
use crate::model::piste::Piste;
use crate::model::skiing::State;

pub fn compute_piste(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    lifts: &HashMap<usize, Lift>,
    gates: &HashMap<usize, Gate>,
    entrances: &mut HashMap<usize, Entrance>,
) {
    let Some(piste) = pistes.get(piste_id) else {
        return;
    };

    let piste_entrances = compute_piste_entrances(piste_id, piste, lifts, gates);

    entrances.extend(piste_entrances);
}

fn compute_piste_entrances(
    piste_id: &usize,
    piste: &Piste,
    lifts: &HashMap<usize, Lift>,
    gates: &HashMap<usize, Gate>,
) -> HashMap<usize, Entrance> {
    let piste = &piste.grid;

    let lifts_iter = lifts
        .iter()
        .filter(|(_, lift)| {
            let position = lift.drop_off.state.position;
            piste.in_bounds(position) && piste[position]
        })
        .map(|(&id, lift)| {
            (
                id,
                Entrance {
                    destination_piste_id: *piste_id,
                    stationary_states: HashSet::from([lift.drop_off.state.stationary()]),
                },
            )
        });

    let gates_iter = gates
        .iter()
        .filter(|(_, gate)| gate.destination_piste == *piste_id)
        .map(|(&id, gate)| {
            (
                id,
                Entrance {
                    destination_piste_id: *piste_id,
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
