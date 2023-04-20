use std::collections::{HashMap, HashSet};

use commons::grid::Grid;
use commons::unsafe_float_ordering;
use network::model::Edge;

use crate::model::skiing::{Event, Plan, State};
use crate::network::skiing::SkiingNetwork;

use network::algorithms::find_best_within_budget::FindBestWithinBudget;

const BUDGET: u64 = 4_000_000;
const MAX_VELOCITY_TARGET: u8 = 6;

pub fn run(terrain: &Grid<f32>, micros: &u128, plans: &mut HashMap<usize, Plan>) {
    for (_, current_plan) in plans.iter_mut() {
        match current_plan {
            Plan::Stationary(state) => {
                *current_plan = new_plan(terrain, micros, state);
            }
            Plan::Moving(events) => {
                let Some(last) = events.last() else {continue};
                if *micros >= last.micros {
                    *current_plan = new_plan(terrain, micros, &last.state);
                }
            }
        }
    }
}

fn new_plan(terrain: &Grid<f32>, micros: &u128, state: &State) -> Plan {
    match find_path(terrain, state) {
        Some(edges) => {
            if edges.is_empty() {
                stop(*state)
            } else {
                Plan::Moving(events(micros, edges))
            }
        }
        None => stop(*state),
    }
}

fn find_path(terrain: &Grid<f32>, state: &State) -> Option<Vec<Edge<State>>> {
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

    let network = SkiingNetwork { terrain };

    network.find_best_within_budget(
        HashSet::from([*state]),
        &|network, state| OrdFloat {
            value: if state.velocity <= MAX_VELOCITY_TARGET {
                -network.terrain[state.position]
            } else {
                f32::NEG_INFINITY
            },
        },
        BUDGET,
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
