use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::find_path::FindPath;

use crate::handlers;
use crate::model::direction::DIRECTIONS;
use crate::model::piste::PisteCosts;
use crate::model::skiing::{Mode, Plan, State};
use crate::network::skiing::SkiingNetwork;
use crate::services::clock;
use crate::systems::planner::events;

pub fn run(
    terrain: &Grid<f32>,
    micros: &u128,
    locations: &HashMap<usize, usize>,
    targets: &HashMap<usize, usize>,
    piste_costs: &HashMap<usize, PisteCosts>,
    reserved: &mut Grid<u8>,
    plans: &mut HashMap<usize, Plan>,
    clock: &mut clock::Service,
    clock_hander: &mut handlers::clock::Handler,
) {
    let mut twizzles = Vec::new();

    {
        let get_cost = |a: &usize, b: &usize| -> Option<u64> {
            let location = locations.get(a)?;
            let piste_costs = piste_costs.get(location)?;
            let target = targets.get(a)?;
            let costs = piste_costs.costs(target)?;
            let plan = plans.get(b)?;
            let state = get_state(plan)?;
            costs.get(state).copied()
        };

        let stationary = plans
            .iter()
            .flat_map(|(id, plan)| get_state(plan).map(|State { position, .. }| (*position, *id)))
            .collect::<HashMap<_, _>>();

        let mut used = HashSet::new();

        for (position, id) in stationary.iter() {
            if used.contains(position) || reserved[position] > 1 {
                continue;
            }
            for neighbour in terrain.neighbours_8(position) {
                if used.contains(&neighbour) || reserved[neighbour] > 1 {
                    continue;
                }
                let Some(neighbour_id) = stationary.get(&neighbour) else {continue};
                if id > neighbour_id {
                    continue;
                }
                let Some(current) = get_cost(id, id) else {continue};
                let Some(potential) = get_cost(id, neighbour_id) else {continue};
                let Some(neighbour_current) = get_cost(neighbour_id, neighbour_id) else {continue};
                let Some(neighbour_potential) = get_cost(neighbour_id, id) else {continue};
                if (potential <= current && neighbour_potential <= neighbour_current)
                    && (potential < current && neighbour_potential < neighbour_current)
                {
                    used.insert(*position);
                    used.insert(neighbour);
                    twizzles.push((*id, *neighbour_id));
                    break;
                }
            }
        }
    }

    for (a, b) in twizzles {
        let plan_a = plans.remove(&a).unwrap();
        let plan_b = plans.remove(&b).unwrap();
        let state_a = get_state(&plan_a).unwrap();
        let state_b = get_state(&plan_b).unwrap();
        let position_a = state_a.position;
        let position_b = state_b.position;

        reserved[position_a] -= 1;
        reserved[position_b] -= 1;

        let network = SkiingNetwork { terrain, reserved };

        let path_a = network.find_path(
            HashSet::from([*state_a]),
            target_states(position_b),
            30_000_000,
            &|_, _| 0,
        );
        let path_b = network.find_path(
            HashSet::from([*state_b]),
            target_states(position_a),
            30_000_000,
            &|_, _| 0,
        );

        if let (Some(path_a), Some(path_b)) = (path_a, path_b) {
            println!("Twizzled {} and {}", a, b);
            plans.insert(a, Plan::Moving(events(micros, path_a)));
            plans.insert(b, Plan::Moving(events(micros, path_b)));
            reserved[position_a] = 2;
            reserved[position_b] = 2;

            println!(
                "Twizzled {} and {}, {} and {}",
                position_a, position_b, reserved[position_a], reserved[position_b]
            );
        }
        // clock_hander.slow(clock);
    }
}

fn get_state(plan: &Plan) -> Option<&State> {
    match plan {
        Plan::Stationary(state) => Some(state),
        Plan::Moving(_) => None,
    }
}

fn target_states(position: XY<u32>) -> HashSet<State> {
    let mut out = HashSet::new();
    for travel_direction in DIRECTIONS {
        out.insert(State {
            position,
            mode: Mode::Walking,
            travel_direction,
        });
        out.insert(State {
            position,
            mode: Mode::Skiing { velocity: 0 },
            travel_direction,
        });
    }
    out
}
