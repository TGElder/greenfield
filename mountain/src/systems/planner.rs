use std::collections::{HashMap, HashSet};
use std::iter::once;

use commons::geometry::XY;
use commons::grid::Grid;
use network::model::Edge;

use crate::model::skiing::{Event, Plan, State};
use crate::model::PisteCosts;
use crate::network::skiing::SkiingNetwork;

use network::algorithms::find_best_within_steps::FindBestWithinSteps;

const MAX_STEPS: u64 = 8;

pub fn run(
    terrain: &Grid<f32>,
    micros: &u128,
    plans: &mut HashMap<usize, Plan>,
    locations: &HashMap<usize, usize>,
    targets: &HashMap<usize, usize>,
    costs: &HashMap<usize, PisteCosts>,
    reserved: &mut Grid<bool>,
) {
    for (id, current_plan) in plans.iter_mut() {
        if let Plan::Moving(ref events) = current_plan {
            if let Some(last) = events.last() {
                if *micros < last.micros {
                    continue;
                }
            }
        }
        free(current_plan, reserved);
        let from = last_state(current_plan);
        *current_plan = match get_costs(id, locations, targets, costs) {
            Some(costs) => new_plan(terrain, micros, from, reserved, costs),
            None => stop(*from),
        };
        reserve(current_plan, reserved);
    }
}

fn free(plan: &Plan, reserved: &mut Grid<bool>) {
    for position in iter_positions(plan) {
        reserved[position] = false
    }
}

fn reserve(plan: &Plan, reserved: &mut Grid<bool>) {
    for position in iter_positions(plan) {
        reserved[position] = true
    }
}

fn iter_positions<'a>(plan: &'a Plan) -> Box<dyn Iterator<Item = XY<u32>> + 'a> {
    match plan {
        Plan::Stationary(state) => Box::new(once(state.position)),
        Plan::Moving(events) => Box::new(
            events
                .iter()
                .map(|event| event.state)
                .map(|state| state.position),
        ),
    }
}

fn last_state(plan: &Plan) -> &State {
    match plan {
        Plan::Stationary(state) => state,
        Plan::Moving(events) => &events.last().unwrap().state,
    }
}

fn get_costs<'a>(
    id: &usize,
    locations: &HashMap<usize, usize>,
    targets: &HashMap<usize, usize>,
    costs: &'a HashMap<usize, PisteCosts>,
) -> Option<&'a HashMap<State, u64>> {
    let location = locations.get(id)?;
    let target = targets.get(id)?;
    let costs = costs.get(location)?;
    costs.costs(target)
}

fn new_plan(
    terrain: &Grid<f32>,
    micros: &u128,
    from: &State,
    reserved: &Grid<bool>,
    costs: &HashMap<State, u64>,
) -> Plan {
    match find_path(terrain, from, reserved, costs) {
        Some(edges) => {
            if edges.is_empty() {
                stop(*from)
            } else {
                Plan::Moving(events(micros, edges))
            }
        }
        None => stop(*from),
    }
}

fn find_path(
    terrain: &Grid<f32>,
    from: &State,
    reserved: &Grid<bool>,
    costs: &HashMap<State, u64>,
) -> Option<Vec<Edge<State>>> {
    let network = SkiingNetwork { terrain, reserved };

    network.find_best_within_steps(
        HashSet::from([*from]),
        &|_, state| match costs.get(state) {
            Some(_) => u64::MAX - costs[state],
            None => 0,
        },
        MAX_STEPS,
    )
}

fn stop(state: State) -> Plan {
    Plan::Stationary(State {
        velocity: 0,
        ..state
    })
}

fn events(start: &u128, edges: Vec<Edge<State>>) -> Vec<Event> {
    let mut out = Vec::with_capacity(edges.len());
    let mut micros = *start;
    let last_i = edges.len() - 1;
    for (i, edge) in edges.into_iter().enumerate() {
        out.push(Event {
            micros,
            state: edge.from,
        });
        micros += edge.cost as u128;
        if i == last_i {
            out.push(Event {
                micros,
                state: edge.to,
            });
        }
    }
    out
}
