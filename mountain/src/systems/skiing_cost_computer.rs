use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::costs_to_targets::CostsToTargets;
use network::algorithms::find_best_within_steps::{self, FindBestWithinSteps};
use network::model::Edge;

use crate::model::direction::DIRECTIONS;
use crate::model::exit::Exit;
use crate::model::piste::{Piste, PisteCosts};
use crate::model::skiing::{Mode, State};
use crate::network::skiing::{SkiingInNetwork, SkiingNetwork};
use crate::network::velocity_encoding::VELOCITY_LEVELS;

pub const MAX_STEPS: u64 = 4;

pub fn run(
    terrain: &Grid<f32>,
    pistes: &HashMap<usize, Piste>,
    exits: &HashMap<usize, Vec<Exit>>,
    distance_costs: &HashMap<usize, PisteCosts>,
    skiing_costs: &mut HashMap<usize, PisteCosts>,
) {
    skiing_costs.clear();
    for (piste_id, piste) in pistes.iter() {
        let Some(exits) = exits.get(piste_id) else {
            continue;
        };
        let costs = compute_costs_for_piste(
            terrain,
            piste_id,
            piste,
            exits,
            distance_costs.get(piste_id).unwrap(),
        );
        skiing_costs.insert(*piste_id, costs);
    }
}

fn compute_costs_for_piste(
    terrain: &Grid<f32>,
    piste_id: &usize,
    piste: &Piste,
    exits: &[Exit],
    distance_costs: &PisteCosts,
) -> PisteCosts {
    let mut out = PisteCosts::new();

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
        let distance_costs = distance_costs.costs(exit_id).unwrap();

        let network = SkiingNetwork {
            terrain,
            is_reserved_fn: &|position| {
                !positions.contains(position) && exit_positions.contains(position)
            },
            is_skiable_edge_fn: &|a, b| match (distance_costs.get(a), distance_costs.get(b)) {
                (Some(to), Some(from)) => to < from,
                _ => false,
            },
        };
        println!("INFO: Computing costs id {} in {}", exit_id, piste_id);
        let costs = compute_costs_for_targets(&network, piste, positions);
        println!(
            "INFO: Computing reachability id {} in {}",
            exit_id, piste_id
        );
        let reachability = compute_reachability_from_costs(&network, piste, &costs);

        let piste_states =
            (piste_positions(piste).count() * DIRECTIONS.len() * (VELOCITY_LEVELS as usize)) as f32;
        println!(
            "INFO: Costs for id {} in {} = {}",
            exit_id,
            piste_id,
            costs.len() as f32 / piste_states
        );
        println!(
            "INFO: Reachability for id {} in {} = {}",
            exit_id,
            piste_id,
            reachability.len() as f32 / piste_states
        );
        out.set_costs(*exit_id, costs);
    }

    out
}

fn piste_positions(piste: &Piste) -> impl Iterator<Item = XY<u32>> + '_ {
    piste.grid.iter().filter(|position| piste.grid[position])
}

fn compute_costs_for_targets(
    network: &SkiingNetwork,
    piste: &Piste,
    targets: &HashSet<XY<u32>>,
) -> HashMap<State, u64> {
    let mut out = HashMap::new();
    let mut cache = HashMap::with_capacity(piste_positions(piste).count());
    for position in piste_positions(piste) {
        for state in skiing_states_for_position(position) {
            let cost = compute_cost_from_state(&state, network, piste, targets, &mut cache);
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
    piste: &Piste,
    targets: &HashSet<XY<u32>>,
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
        compute_cost_from_state(&zero_state, network, piste, targets, cache)?;
    }

    let result = network.find_best_within_steps(
        HashSet::from([*from]),
        &mut |_, state| {
            if state.position == from.position {
                return None;
            }

            compute_cost_from_state(state, network, piste, targets, cache)
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

fn compute_reachability_from_costs(
    network: &SkiingNetwork,
    piste: &Piste,
    costs: &HashMap<State, u64>,
) -> HashSet<State> {
    let positions = piste_positions(piste).collect();
    let in_network = SkiingInNetwork::for_states(network, &positions);
    let targets = costs.keys().copied().collect();
    in_network
        .costs_to_targets(&targets, Some(MAX_STEPS))
        .keys()
        .copied()
        .collect()
}
