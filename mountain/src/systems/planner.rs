use std::collections::{HashMap, HashSet};
use std::iter::once;

use commons::geometry::XY;
use commons::grid::Grid;
use commons::unsafe_float_ordering;
use network::model::Edge;

use crate::model::skiing::{Event, Plan, State};
use crate::network::skiing::SkiingNetwork;

use network::algorithms::find_best_within_steps::FindBestWithinSteps;

const MAX_STEPS: u64 = 8;
const MAX_VELOCITY_TARGET: u8 = 6;

pub fn run(
    terrain: &Grid<f32>,
    micros: &u128,
    plans: &mut HashMap<usize, Plan>,
    reserved: &mut Grid<bool>,
) {
    for (_, current_plan) in plans.iter_mut() {
        if let Plan::Moving(ref events) = current_plan {
            if let Some(last) = events.last() {
                if *micros < last.micros {
                    continue;
                }
            }
        }
        free(current_plan, reserved);
        *current_plan = new_plan(terrain, micros, current_plan, reserved);
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

fn new_plan(terrain: &Grid<f32>, micros: &u128, plan: &Plan, reserved: &Grid<bool>) -> Plan {
    let from = last_state(plan);
    match find_path(terrain, from, reserved) {
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

fn last_state(plan: &Plan) -> &State {
    match plan {
        Plan::Stationary(state) => state,
        Plan::Moving(events) => &events.last().unwrap().state,
    }
}

fn find_path(terrain: &Grid<f32>, from: &State, reserved: &Grid<bool>) -> Option<Vec<Edge<State>>> {
    #[derive(Debug)]
    struct OrdFloat {
        value: f32,
    }

    impl Ord for OrdFloat {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            unsafe_float_ordering(&self.value, &other.value)
        }
    }

    impl PartialOrd for OrdFloat {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.value.partial_cmp(&other.value)
        }
    }

    impl PartialEq for OrdFloat {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    impl Eq for OrdFloat {}

    let network = SkiingNetwork { terrain, reserved };

    network.find_best_within_steps(
        HashSet::from([*from]),
        &|network, state| OrdFloat {
            value: if state.velocity <= MAX_VELOCITY_TARGET {
                -network.terrain[state.position]
            } else {
                f32::NEG_INFINITY
            },
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
