use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY};
use commons::grid::Grid;
use network::algorithms::find_best_within_steps::{self, FindBestWithinSteps};
use network::model::Edge;

use crate::model::direction::DIRECTIONS;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
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
    entrances: &HashMap<usize, Entrance>,
) {
    for (piste_id, piste) in pistes.iter() {
        let costs = compute_costs_for_piste(
            terrain,
            piste_id,
            piste,
            lifts,
            entrances,
            distance_costs.get(piste_id).unwrap(),
        );
        skiing_costs.insert(*piste_id, costs);
    }
}

fn compute_costs_for_piste(
    terrain: &Grid<f32>,
    piste_id: &usize,
    piste: &Piste,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
    distance_costs: &PisteCosts,
) -> PisteCosts {
    let mut out = PisteCosts::new();

    let lift_positions = lifts
        .values()
        .map(|lift| lift.pick_up.position)
        .collect::<HashSet<_>>();

    let grid = &piste.grid;

    let lifts_iter = lifts
        .iter()
        .map(|(lift_id, lift)| (lift_id, vec![lift.pick_up.position]));

    let entrances_iter = entrances
        .iter()
        .filter(|(_, entrance)| entrance.piste != *piste_id)
        .map(|(entrance_id, entrance)| {
            (
                entrance_id,
                (entrance.from.x..=entrance.to.x)
                    .flat_map(|x| (entrance.from.y..=entrance.to.y).map(move |y| xy(x, y)))
                    .filter(|position| grid.in_bounds(position))
                    .collect::<Vec<_>>(),
            )
        });

    let candidates = lifts_iter
        .chain(entrances_iter)
        .map(|(id, positions)| {
            (
                id,
                positions
                    .into_iter()
                    .filter(|position| grid.in_bounds(position) && grid[position])
                    .collect::<HashSet<_>>(),
            )
        })
        .filter(|(_, positions)| !positions.is_empty());

    for (id, positions) in candidates {
        let distance_costs = distance_costs.costs(id).unwrap();

        let network = SkiingNetwork {
            terrain,
            is_reserved_fn: &|position| {
                !positions.contains(position) && lift_positions.contains(position)
            },
            is_skiable_edge_fn: &|a, b| match (distance_costs.get(a), distance_costs.get(b)) {
                (Some(to), Some(from)) => to < from,
                _ => false,
            },
        };
        let costs = compute_costs_for_targets(&positions, piste, &network);

        let coverage = costs.len() as f32
            / (piste_positions(piste).count() * DIRECTIONS.len() * (VELOCITY_LEVELS as usize + 1))
                as f32;
        println!("INFO: Coverage for id {} = {}", id, coverage);
        out.set_costs(*id, costs);
    }

    out
}

fn piste_positions(piste: &Piste) -> impl Iterator<Item = XY<u32>> + '_ {
    piste.grid.iter().filter(|position| piste.grid[position])
}

fn compute_costs_for_targets(
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
