use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;

use crate::model::skiing::{Mode, State};
use crate::model::{Lift, Piste, PisteCosts, DIRECTIONS};
use crate::network::skiing::{SkiingInNetwork, SkiingNetwork};
use crate::network::velocity_encoding::encode_velocity;
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

    for (
        lift,
        Lift {
            from,
            max_entry_velocity,
            ..
        },
    ) in lifts
    {
        let grid = &piste.grid;
        if grid.in_bounds(from) && grid[from] {
            let costs = compute_costs_for_position(&network, from, max_entry_velocity);
            let mut adjusted = HashMap::new();
            for (state, cost) in costs.iter() {
                let brake_state = State {
                    mode: Mode::Skiing { velocity: 0 },
                    ..*state
                };
                if let Some(_) = costs.get(&brake_state) {
                    adjusted.insert(*state, *cost);
                }
            }
            let coverage =
                adjusted.len() as f32 / (piste_positions.len() * DIRECTIONS.len() * 9) as f32;
            println!("INFO: Coverage for lift {} = {}", lift, coverage);
            out.set_costs(*lift, adjusted)
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
    max_velocity: &f32,
) -> HashMap<State, u64> {
    let states_iter = skiing_states_for_position(*position, max_velocity);
    network.costs_to_target(&HashSet::from_iter(states_iter))
}

fn skiing_states_for_position(
    position: XY<u32>,
    max_velocity: &f32,
) -> impl Iterator<Item = State> {
    encode_velocity(max_velocity)
        .into_iter()
        .flat_map(move |max_velocity| {
            DIRECTIONS
                .iter()
                .copied()
                .flat_map(move |travel_direction| {
                    (0..=max_velocity).map(move |velocity| State {
                        position,
                        mode: Mode::Skiing { velocity },
                        travel_direction,
                    })
                })
        })
}
