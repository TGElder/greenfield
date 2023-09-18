use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;

use crate::model::direction::DIRECTIONS;
use crate::model::lift::Lift;
use crate::model::piste::{Piste, PisteCosts};
use crate::model::skiing::{Mode, State};
use crate::network::skiing::{SkiingInNetwork, SkiingNetwork};
use crate::network::velocity_encoding::VELOCITY_LEVELS;
use network::algorithms::costs_to_target::CostsToTarget;

pub fn run(
    terrain: &Grid<f32>,
    pistes: &HashMap<usize, Piste>,
    piste_costs: &HashMap<usize, PisteCosts>,
    true_costs: &mut HashMap<usize, PisteCosts>,
    lifts: &HashMap<usize, Lift>,
) {
    for (piste_index, piste) in pistes.iter() {
        let costs = compute_costs(terrain, piste, lifts, piste_costs.get(piste_index).unwrap());
        true_costs.insert(*piste_index, costs);
    }
}

fn compute_costs(
    terrain: &Grid<f32>,
    piste: &Piste,
    lifts: &HashMap<usize, Lift>,
    distance_costs: &PisteCosts,
) -> PisteCosts {
    let mut out = PisteCosts::new();

    let piste_positions = piste_positions(piste);
    let lift_positions = lifts.values().map(|lift| lift.from).collect::<HashSet<_>>();
    let mut reserved = terrain.map(|position, _| lift_positions.contains(&position));

    for (lift, Lift { from, .. }) in lifts {
        let grid = &piste.grid;
        if grid.in_bounds(from) && grid[from] {
            let distance_costs = distance_costs.costs(lift).unwrap();

            reserved[from] = false;
            let network = SkiingNetwork {
                terrain,
                reserved: &reserved,
                distance_costs,
            };
            let in_network = SkiingInNetwork::for_positions(&network, &piste_positions);
            reserved[from] = true;

            let costs = compute_costs_for_position(&in_network, from);

            let coverage = costs.len() as f32
                / (piste_positions.len() * DIRECTIONS.len() * (VELOCITY_LEVELS as usize + 1))
                    as f32;
            println!("INFO: Coverage for lift {} = {}", lift, coverage);
            out.set_costs(*lift, costs);
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

fn compute_costs_for_position(
    network: &SkiingInNetwork,
    position: &XY<u32>,
) -> HashMap<State, u64> {
    let states_iter = skiing_states_for_position(*position);
    network.costs_to_target(&HashSet::from_iter(states_iter))
}

fn skiing_states_for_position(position: XY<u32>) -> impl Iterator<Item = State> {
    DIRECTIONS
        .iter()
        .copied()
        .flat_map(move |travel_direction| {
            (0..=VELOCITY_LEVELS).map(move |velocity| State {
                position,
                mode: Mode::Skiing { velocity },
                travel_direction,
            })
        })
}
