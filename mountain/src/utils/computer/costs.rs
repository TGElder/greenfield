use std::collections::{HashMap, HashSet};

use crate::model::ability::ABILITIES;
use crate::model::costs::Costs;
use crate::model::direction::DIRECTIONS;
use crate::model::exit::Exit;
use crate::model::piste::Piste;
use crate::model::reservation::Reservation;
use crate::network::skiing::{SkiingNetwork, StationaryNetwork};
use commons::geometry::XY;
use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;
use network::algorithms::costs_to_targets::CostsToTargets;

pub fn compute_piste(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    terrain: &Grid<f32>,
    exits: &HashMap<usize, Vec<Exit>>,
    reservations: &Grid<HashMap<usize, Reservation>>,
    costs: &mut HashMap<usize, Costs>,
) {
    costs.remove(piste_id);

    let Some(piste) = pistes.get(piste_id) else {
        return;
    };
    let Some(exits) = exits.get(piste_id) else {
        return;
    };

    let piste_costs = compute_costs(terrain, piste, exits, reservations);

    costs.insert(*piste_id, piste_costs);
}

fn compute_costs(
    terrain: &Grid<f32>,
    piste: &Piste,
    exits: &[Exit],
    reservations: &Grid<HashMap<usize, Reservation>>,
) -> Costs {
    let mut out = Costs::new();

    for Exit {
        id: exit_id,
        states,
        ..
    } in exits
    {
        let min_z = states
            .iter()
            .map(|state| state.position)
            .map(|position| terrain[position])
            .min_by(unsafe_ordering)
            .unwrap();
        for ability in ABILITIES {
            let network = SkiingNetwork {
                terrain,
                ability,
                is_accessible_fn: &|position| {
                    terrain[position] >= min_z
                        && !reservations[position]
                            .iter()
                            .filter(|(id, _)| *id != exit_id)
                            .map(|(_, reservation)| reservation)
                            .any(|reservation| *reservation == Reservation::Structure)
                },
                is_valid_edge_fn: &|_, _| true,
            };
            let network = StationaryNetwork::for_positions(&network, &piste_positions(piste));

            let costs = {
                let network = &network;
                network.costs_to_targets(states, None)
            };
            let coverage =
                costs.len() as f32 / (piste_positions(piste).len() * DIRECTIONS.len()) as f32;
            println!(
                "INFO: Coverage for id {}, {:?} = {}",
                exit_id, ability, coverage
            );
            out.set_costs(*exit_id, ability, costs)
        }
    }

    out
}

fn piste_positions(piste: &Piste) -> HashSet<XY<u32>> {
    piste
        .grid
        .iter()
        .filter(|position| piste.grid[position])
        .collect::<HashSet<_>>()
}
