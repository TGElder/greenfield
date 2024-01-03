use std::collections::{HashMap, HashSet};
use std::iter::once;

use crate::model::direction::DIRECTIONS;
use crate::model::exit::Exit;
use crate::model::piste::{Costs, Piste};
use crate::model::skiing::{Mode, State};
use crate::network::distance::DistanceNetwork;
use crate::network::velocity_encoding::VELOCITY_LEVELS;
use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::costs_to_targets::CostsToTargets;

pub fn run(
    terrain: &Grid<f32>,
    pistes: &HashMap<usize, Piste>,
    exits: &HashMap<usize, Vec<Exit>>,
    distance_costs: &mut HashMap<usize, Costs>,
) {
    distance_costs.clear();
    for (piste_id, piste) in pistes.iter() {
        let Some(exits) = exits.get(piste_id) else {
            continue;
        };
        let costs = compute_costs(terrain, piste_id, piste, exits);
        distance_costs.insert(*piste_id, costs);
    }
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
        let network = DistanceNetwork {
            terrain,
            piste,
            can_visit: &|position| {
                positions.contains(position) || !exit_positions.contains(position)
            },
        };

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
    network: &DistanceNetwork,
    targets: &HashSet<XY<u32>>,
) -> HashMap<State, u64> {
    let distances = network.costs_to_targets(targets, None);
    to_costs(distances)
}

fn to_costs(mut distances: HashMap<XY<u32>, u64>) -> HashMap<State, u64> {
    distances
        .drain()
        .flat_map(|(position, distance)| {
            states_for_position(position).map(move |state| (state, distance))
        })
        .collect()
}

fn states_for_position(position: XY<u32>) -> impl Iterator<Item = State> {
    DIRECTIONS.into_iter().flat_map(move |travel_direction| {
        modes().map(move |mode| State {
            position,
            mode,
            travel_direction,
        })
    })
}

fn modes() -> impl Iterator<Item = Mode> {
    (0..VELOCITY_LEVELS)
        .map(|velocity| Mode::Skiing { velocity })
        .chain(once(Mode::Walking))
}
