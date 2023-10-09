use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::find_best_within_steps::FindBestWithinSteps;

use crate::model::direction::DIRECTIONS;
use crate::model::lift::Lift;
use crate::model::piste::{Piste, PisteCosts};
use crate::model::skiing::{Mode, State};
use crate::network::skiing::SkiingNetwork;
use crate::network::velocity_encoding::VELOCITY_LEVELS;

pub fn run(
    terrain: &Grid<f32>,
    pistes: &HashMap<usize, Piste>,
    distance_costs: &HashMap<usize, PisteCosts>,
    skiing_costs: &mut HashMap<usize, PisteCosts>,
    lifts: &HashMap<usize, Lift>,
) {
    for (piste_index, piste) in pistes.iter() {
        let costs = compute_costs(
            terrain,
            piste,
            lifts,
            distance_costs.get(piste_index).unwrap(),
        );
        skiing_costs.insert(*piste_index, costs);
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

            let mut cache = HashMap::new();
            let mut costs = HashMap::new();

            for position in piste_positions.iter() {
                for state in skiing_states_for_position(*position) {
                    let cost = compute_cost_for_state(
                        &mut HashSet::new(),
                        &mut cache,
                        &network,
                        &state,
                        from,
                        &piste_positions,
                    );
                    if let Some(cost) = cost {
                        costs.insert(state, cost);
                    }
                }
            }

            reserved[from] = true;

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

fn compute_cost_for_state(
    requested: &mut HashSet<State>,
    costs: &mut HashMap<State, Option<u64>>,
    network: &SkiingNetwork,
    from: &State,
    target: &XY<u32>,
    piste_positions: &HashSet<XY<u32>>,
) -> Option<u64> {
    // println!("Computing {:?}", from);

    if from.position == *target {
        // println!("Target");
        return Some(0);
    }

    if let Some(cost) = costs.get(from) {
        // println!("Cached");
        return *cost;
    }

    if requested.contains(from) {
        panic!("Infinite loop!");
    } else {
        requested.insert(*from);
    }

    let path = network.find_best_within_steps(
        HashSet::from([*from]),
        &mut |_, state| {
            if state.position == from.position {
                return None;
            }

            let Some(cost) =
                compute_cost_for_state(requested, costs, network, state, target, piste_positions)
            else {
                return None;
            };

            // check for forbidden tiles
            if cost != 0 {
                // goal tile is never forbidden

                if is_white_tile(&state.position) {
                    return None;
                }

                if let Mode::Skiing { velocity } = state.mode {
                    if velocity != 0 {
                        return None;
                    }
                }
            }

            Some(cost)
        },
        &mut |state| piste_positions.contains(&state.position),
        8,
    );

    let Some(path) = path else {
        // println!("Computed {:?}", from);
        costs.insert(*from, None);
        return None;
    };

    if path.is_empty() {
        panic!("Path to self");
    }

    let path_cost = path.iter().map(|edge| edge.cost as u64).sum::<u64>();
    let final_cost = compute_cost_for_state(
        requested,
        costs,
        network,
        &path.last().unwrap().to,
        target,
        piste_positions,
    )
    .unwrap();

    let out = Some(path_cost + final_cost);
    // println!("Computed {:?}", from);
    costs.insert(*from, out);
    out
}

fn is_white_tile(position: &XY<u32>) -> bool {
    position.x % 2 == position.y % 2
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
