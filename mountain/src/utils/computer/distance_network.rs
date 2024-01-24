use std::collections::{HashMap, HashSet};

use crate::model::direction::DIRECTIONS;
use crate::model::exit::Exit;
use crate::model::piste::{Costs, Piste};
use crate::model::skiing::State;
use crate::network::distance::DistanceNetwork;
use crate::network::skiing::{SkiingInNetwork, SkiingNetwork};
use crate::network::velocity_encoding::VELOCITY_LEVELS;
use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::costs_to_targets::CostsToTargets;

pub fn compute_piste(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    terrain: &Grid<f32>,
    exits: &HashMap<usize, Vec<Exit>>,
    distance_costs: &mut HashMap<usize, Costs>,
) {
    distance_costs.remove(piste_id);

    let Some(piste) = pistes.get(piste_id) else {
        return;
    };
    let Some(exits) = exits.get(piste_id) else {
        return;
    };

    let costs = compute_costs(terrain, piste_id, piste, exits);

    distance_costs.insert(*piste_id, costs);
}

fn compute_costs(terrain: &Grid<f32>, piste_id: &usize, piste: &Piste, exits: &[Exit]) -> Costs {
    let mut out = Costs::new();

    let exit_positions = exits
        .iter()
        .filter(|Exit { id, .. }| id != piste_id)
        .flat_map(|Exit { positions, .. }| positions)
        .filter(|position| piste.grid.in_bounds(*position))
        .filter(|position| piste.grid[*position])
        .collect::<HashSet<_>>();

    for Exit {
        id: exit_id,
        positions,
    } in exits
    {
        let network = SkiingNetwork {
            terrain,
            is_accessible_fn: &|position| {
                positions.contains(position) || !exit_positions.contains(position)
            },
            is_skiable_edge_fn: &|_, _| true,
        };
        let network = SkiingInNetwork::for_positions(&network, positions);

        let costs = compute_costs_for_targets(&network, positions);
        let coverage = costs.len() as f32
            / (piste_positions(piste).len() * DIRECTIONS.len() * (VELOCITY_LEVELS as usize + 1))
                as f32;
        println!("INFO: Coverage for id {} = {}", exit_id, coverage);
        out.set_costs(*exit_id, costs)
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

fn compute_costs_for_targets(
    network: &SkiingInNetwork,
    targets: &HashSet<XY<u32>>,
) -> HashMap<State, u64> {
    network.costs_to_targets(
        &targets
            .iter()
            .flat_map(|target| states_for_position(*target))
            .collect(),
        None,
    )
}

fn states_for_position(position: XY<u32>) -> impl Iterator<Item = State> {
    DIRECTIONS.into_iter().map(move |travel_direction| State {
        position,
        velocity: 0,
        travel_direction,
    })
}
