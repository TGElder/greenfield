use std::collections::{HashMap, HashSet};
use std::iter::once;

use crate::model::direction::DIRECTIONS;
use crate::model::lift::Lift;
use crate::model::piste::{Piste, PisteCosts};
use crate::model::skiing::{Mode, State};
use crate::network::distance::DistanceNetwork;
use crate::network::velocity_encoding::VELOCITY_LEVELS;
use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::costs_to_target::CostsToTarget;

pub fn run(
    terrain: &Grid<f32>,
    pistes: &HashMap<usize, Piste>,
    distance_costs: &mut HashMap<usize, PisteCosts>,
    lifts: &HashMap<usize, Lift>,
) {
    for (piste_index, piste) in pistes.iter() {
        let costs = compute_costs(terrain, piste, lifts);
        distance_costs.insert(*piste_index, costs);
    }
}

fn compute_costs(terrain: &Grid<f32>, piste: &Piste, lifts: &HashMap<usize, Lift>) -> PisteCosts {
    let mut out = PisteCosts::new();

    let lift_positions = lifts.values().map(|lift| lift.from).collect::<HashSet<_>>();

    let network = DistanceNetwork {
        terrain,
        piste,
        can_visit: &|position| !lift_positions.contains(position),
    };

    for (lift, Lift { from, .. }) in lifts {
        let grid = &piste.grid;
        if grid.in_bounds(from) && grid[from] {
            let costs = compute_costs_to_position(&network, from);
            let coverage = costs.len() as f32
                / (piste_positions(piste).len() * DIRECTIONS.len() * (VELOCITY_LEVELS as usize + 1))
                    as f32;
            println!("INFO: Coverage for lift {} = {}", lift, coverage);
            out.set_costs(*lift, costs)
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

fn compute_costs_to_position(network: &DistanceNetwork, position: &XY<u32>) -> HashMap<State, u64> {
    let distances = network.costs_to_target(&HashSet::from([*position]));
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
