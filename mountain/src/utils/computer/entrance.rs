use std::collections::{HashMap, HashSet};

use commons::grid::Grid;

use crate::model::direction::DIRECTIONS;
use crate::model::entrance::Entrance;
use crate::model::gate::Gate;
use crate::model::piste::Piste;
use crate::model::skiing::State;

pub fn compute_piste(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    gates: &HashMap<usize, Gate>,
    terrain: &Grid<f32>,
    entrances: &mut HashMap<usize, Entrance>,
) {
    let Some(piste) = pistes.get(piste_id) else {
        return;
    };

    let piste_entrances = compute_piste_entrances(piste_id, piste, gates, terrain);

    entrances.extend(piste_entrances);
}

fn compute_piste_entrances(
    piste_id: &usize,
    piste: &Piste,
    gates: &HashMap<usize, Gate>,
    terrain: &Grid<f32>,
) -> HashMap<usize, Entrance> {
    let piste = &piste.grid;

    gates
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
                    altitude_meters: gate
                        .footprint
                        .iter()
                        .map(|position| terrain[position])
                        .sum::<f32>()
                        / gate.footprint.iter().count() as f32,
                },
            )
        })
        .collect::<HashMap<_, _>>()
}
