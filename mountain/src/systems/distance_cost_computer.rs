use std::collections::{HashMap, HashSet};
use std::iter::once;

use crate::model::direction::DIRECTIONS;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::piste::{Piste, PisteCosts};
use crate::model::skiing::{Mode, State};
use crate::network::distance::DistanceNetwork;
use crate::network::velocity_encoding::VELOCITY_LEVELS;
use commons::geometry::{xy, XY};
use commons::grid::Grid;
use network::algorithms::costs_to_target::CostsToTarget;

pub fn run(
    terrain: &Grid<f32>,
    pistes: &HashMap<usize, Piste>,
    distance_costs: &mut HashMap<usize, PisteCosts>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
) {
    for (piste_id, piste) in pistes.iter() {
        let costs = compute_costs(terrain, piste_id, piste, lifts, entrances);
        distance_costs.insert(*piste_id, costs);
    }
}

fn compute_costs(
    terrain: &Grid<f32>,
    piste_id: &usize,
    piste: &Piste,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
) -> PisteCosts {
    let mut out = PisteCosts::new();

    let lift_positions = lifts
        .values()
        .map(|lift| lift.pick_up.position)
        .collect::<HashSet<_>>();

    let network = DistanceNetwork {
        terrain,
        piste,
        can_visit: &|position| !lift_positions.contains(position),
    };

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
        let costs = compute_costs_to_positions(&network, &positions);
        let coverage = costs.len() as f32
            / (piste_positions(piste).len() * DIRECTIONS.len() * (VELOCITY_LEVELS as usize + 1))
                as f32;
        println!("INFO: Coverage for id {} = {}", id, coverage);
        out.set_costs(*id, costs)
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

fn compute_costs_to_positions(
    network: &DistanceNetwork,
    positions: &HashSet<XY<u32>>,
) -> HashMap<State, u64> {
    let distances = network.costs_to_target(positions);
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
