use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::find_best_within_steps::{self, FindBestWithinSteps};
use network::model::Edge;

use crate::model::direction::DIRECTIONS;
use crate::model::lift::{self, Lift};
use crate::model::piste::{Piste, PisteCosts};
use crate::model::skiing::{Mode, State};
use crate::network::skiing::SkiingNetwork;
use crate::network::velocity_encoding::VELOCITY_LEVELS;

pub const MAX_STEPS: u64 = 4;

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

    let lift_positions = lifts
        .values()
        .flat_map(|lift| lift.nodes.iter())
        .flat_map(|node| match node.from_action {
            Some(lift::Action::PickUp(position)) => Some(position),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let mut reserved = terrain.map(|position, _| lift_positions.contains(&position));

    for (lift, Lift { nodes, .. }) in lifts {
        let grid = &piste.grid;
        let from = nodes
            .iter()
            .flat_map(|node| match node.from_action {
                Some(lift::Action::PickUp(position)) => Some(position),
                _ => None,
            })
            .filter(|position| grid.in_bounds(position) && grid[position])
            .collect::<HashSet<_>>();

        let distance_costs = distance_costs.costs(lift).unwrap();

        for from in from.iter() {
            reserved[from] = false;
        }
        let network = SkiingNetwork {
            terrain,
            reserved: &reserved,
            distance_costs,
        };
        let costs = compute_costs_for_target(&from, piste, &network);
        for from in from.iter() {
            reserved[from] = true;
        }

        let coverage = costs.len() as f32
            / (piste_positions(piste).count() * DIRECTIONS.len() * (VELOCITY_LEVELS as usize + 1))
                as f32;
        println!("INFO: Coverage for lift {} = {}", lift, coverage);
        out.set_costs(*lift, costs);
    }

    out
}

fn piste_positions(piste: &Piste) -> impl Iterator<Item = XY<u32>> + '_ {
    piste.grid.iter().filter(|position| piste.grid[position])
}

fn compute_costs_for_target(
    targets: &HashSet<XY<u32>>,
    piste: &Piste,
    network: &SkiingNetwork,
) -> HashMap<State, u64> {
    let mut out = HashMap::new();
    let mut cache = HashMap::with_capacity(piste_positions(piste).count());
    for position in piste_positions(piste) {
        for state in skiing_states_for_position(position) {
            let cost = compute_cost_from_state(&state, network, targets, piste, &mut cache);
            if let Some(cost) = cost {
                out.insert(state, cost);
            }
        }
    }
    out
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

fn compute_cost_from_state(
    from: &State,
    network: &SkiingNetwork,
    targets: &HashSet<XY<u32>>,
    piste: &Piste,
    cache: &mut HashMap<State, Option<u64>>,
) -> Option<u64> {
    if targets.contains(&from.position) {
        return Some(0);
    }

    if is_white_tile(&from.position) {
        return None;
    }

    if let Some(cost) = cache.get(from) {
        return *cost;
    }

    if !matches!(from.mode, Mode::Skiing { velocity: 0 }) {
        let zero_state = State {
            mode: Mode::Skiing { velocity: 0 },
            ..*from
        };
        compute_cost_from_state(&zero_state, network, targets, piste, cache)?;
    }

    let result = network.find_best_within_steps(
        HashSet::from([*from]),
        &mut |_, state| {
            if state.position == from.position {
                return None;
            }

            compute_cost_from_state(state, network, targets, piste, cache)
        },
        &mut |state| piste.grid.in_bounds(state.position) && piste.grid[state.position],
        MAX_STEPS,
    );

    let out = result.map(|find_best_within_steps::Result { score, path }| score + path_cost(&path));
    cache.insert(*from, out);
    out
}

fn is_white_tile(position: &XY<u32>) -> bool {
    position.x % 2 == position.y % 2
}

fn path_cost(edges: &[Edge<State>]) -> u64 {
    edges.iter().map(|edge| edge.cost as u64).sum()
}
