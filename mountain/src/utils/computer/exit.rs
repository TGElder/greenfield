use std::collections::{HashMap, HashSet};

use crate::model::direction::DIRECTIONS;
use crate::model::exit::Exit;
use crate::model::gate::Gate;
use crate::model::piste::Piste;
use crate::model::skiing::State;

pub fn compute_piste(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    gates: &HashMap<usize, Gate>,
    exits: &mut HashMap<usize, Exit>,
) {
    let Some(piste) = pistes.get(piste_id) else {
        return;
    };

    let piste_exits = compute_piste_exits(piste_id, piste, gates);

    exits.extend(piste_exits);
}

fn compute_piste_exits(
    piste_id: &usize,
    piste: &Piste,
    gates: &HashMap<usize, Gate>,
) -> HashMap<usize, Exit> {
    let piste = &piste.grid;

    gates
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
        })
        .collect::<HashMap<_, _>>()
}
