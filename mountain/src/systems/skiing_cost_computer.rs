use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::find_best_within_steps::FindBestWithinSteps;

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
    distance_costs: &HashMap<usize, PisteCosts>,
    skiing_costs: &mut HashMap<usize, PisteCosts>,
    lifts: &HashMap<usize, Lift>,
) {
    for (piste_index, piste) in pistes.iter() {
        let costs = compute_costs_for_piste(
            terrain,
            piste,
            lifts,
            distance_costs.get(piste_index).unwrap(),
        );
        skiing_costs.insert(*piste_index, costs);
    }
}

fn compute_costs_for_piste(
    terrain: &Grid<f32>,
    piste: &Piste,
    lifts: &HashMap<usize, Lift>,
    distance_costs: &PisteCosts,
) -> PisteCosts {
    let mut out = PisteCosts::new();

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
            let costs = compute_costs_for_target(from, piste, &network);
            reserved[from] = true;

            let coverage = costs.len() as f32
                / (piste_positions(piste).count() * DIRECTIONS.len() * (VELOCITY_LEVELS as usize + 1))
                    as f32;
            println!("INFO: Coverage for lift {} = {}", lift, coverage);
            out.set_costs(*lift, costs);
        }
    }

    out
}

fn piste_positions<'a>(piste: &'a Piste) -> impl Iterator<Item = XY<u32>> + 'a {
    piste
        .grid
        .iter()
        .filter(|position| piste.grid[position])
}

fn compute_costs_for_target(
    target: &XY<u32>,
    piste: &Piste,
    network: &SkiingNetwork,
) -> HashMap<State, u64> {
    let mut out = HashMap::new();
    let mut cache = HashMap::with_capacity(piste_positions(piste).count());
    for position in piste_positions(piste) {
        for state in skiing_states_for_position(position) {
            let cost = compute_cost_for_state(&state, network, target, piste, &mut cache);
            if let Some(cost) = cost {
                out.insert(state, cost);
            }
        }
    }
    out
}

fn compute_cost_for_state(
    state: &State,
    network: &SkiingNetwork,
    target: &XY<u32>,
    piste: &Piste,
    cache: &mut HashMap<State, Option<u64>>,
) -> Option<u64> {
    if state.position == *target {
        return Some(0);
    }

    if is_white_tile(&state.position) {
        return None;
    }

    if let Mode::Skiing { velocity } = state.mode {
        if velocity != 0 {
            return None;
        }
    }

    if let Some(cost) = cache.get(state) {
        return *cost;
    }

    let path = network.find_best_within_steps(
        HashSet::from([*state]),
        &mut |_, focus| {
            if focus.position == state.position {
                return None;
            }

            compute_cost_for_state(state, network, target, piste, cache)
        },
        &mut |state| piste.grid.in_bounds(state.position) && piste.grid[state.position],
        4
    );

    


}



fn compute_costs_for_position(
    network: &SkiingInNetwork,
    position: &XY<u32>,
) -> HashMap<State, u64> {
    let states_iter = skiing_states_for_position(*position);
    network.costs_to_target(&HashSet::from_iter(states_iter))
}

fn is_white_tile(position: &XY<u32>) -> bool {
    position.x % 2 == position.y % 2
}

fn skiing_states_for_position(position: XY<u32>) -> impl Iterator<Item = State> {
    DIRECTIONS
        .iter()
        .copied()
        .flat_map(move |travel_direction| {
            (0..VELOCITY_LEVELS).map(move |velocity| State {
                position,
                mode: Mode::Skiing { velocity },
                travel_direction,
            })
        })
}
