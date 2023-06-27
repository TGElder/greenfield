use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;

use crate::model::skiing::State;
use crate::model::{Lift, Piste, PisteCosts, DIRECTIONS};
use crate::network::skiing::{SkiingInNetwork, SkiingNetwork};
use network::algorithms::costs_to_target::CostsToTarget;

pub fn run(
    terrain: &Grid<f32>,
    pistes: &HashMap<usize, Piste>,
    piste_costs: &mut HashMap<usize, PisteCosts>,
    lifts: &HashMap<usize, Lift>,
) {
    for (piste_index, piste) in pistes.iter() {
        let costs = compute_costs(terrain, piste, lifts);
        piste_costs.insert(*piste_index, costs);
    }
}

fn compute_costs(terrain: &Grid<f32>, piste: &Piste, lifts: &HashMap<usize, Lift>) -> PisteCosts {
    let mut out = PisteCosts::new();

    let network = SkiingNetwork {
        terrain,
        reserved: &terrain.map(|_, _| false),
    };
    let piste_positions = piste_positions(piste);
    let network = SkiingInNetwork::for_positions(&network, &piste_positions);

    for (lift, Lift { from, .. }) in lifts {
        let grid = &piste.grid;
        if grid.in_bounds(from) && grid[from] {
            let costs = compute_costs_for_position(&network, from);
            let coverage =
                costs.len() as f32 / (piste_positions.len() * DIRECTIONS.len() * 8) as f32;
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

fn compute_costs_for_position(
    network: &SkiingInNetwork,
    position: &XY<u32>,
) -> HashMap<State, u64> {
    let states_iter = states_for_position(*position);
    network.costs_to_target(&HashSet::from_iter(states_iter))
}

fn states_for_position(position: XY<u32>) -> impl Iterator<Item = State> {
    DIRECTIONS
        .iter()
        .copied()
        .map(move |travel_direction| State {
            position,
            velocity: 0,
            travel_direction,
        })
}
