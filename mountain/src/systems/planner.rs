use std::collections::{HashMap, HashSet};
use std::iter::once;

use commons::geometry::XY;
use commons::grid::Grid;
use network::model::Edge;

use crate::model::piste::PisteCosts;
use crate::model::skiing::{Event, Mode, Plan, State};
use crate::network::skiing::SkiingNetwork;

use network::algorithms::find_best_within_steps::FindBestWithinSteps;

const MAX_STEPS: u64 = 8;

pub struct System {
    finished: HashVec,
}

pub struct Parameters<'a> {
    pub terrain: &'a Grid<f32>,
    pub micros: &'a u128,
    pub plans: &'a mut HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub targets: &'a HashMap<usize, usize>,
    pub costs: &'a HashMap<usize, PisteCosts>,
    pub true_costs: &'a HashMap<usize, PisteCosts>,
    pub reserved: &'a mut Grid<bool>,
}

impl System {
    pub fn new() -> System {
        System {
            finished: HashVec::new(),
        }
    }

    pub fn run(
        &mut self,
        Parameters {
            terrain,
            micros,
            plans,
            locations,
            targets,
            costs,
            true_costs,
            reserved,
        }: Parameters<'_>,
    ) {
        self.add_new_finished(plans, micros);

        self.finished.retain(|id| {
            let Some(current_plan) = plans.get_mut(id) else {
                return false;
            };

            free(current_plan, reserved);
            let from = last_state(current_plan);
            *current_plan = match (
                get_costs(id, locations, targets, costs),
                get_costs(id, locations, targets, true_costs),
            ) {
                (Some(costs), Some(true_costs)) => {
                    new_plan(terrain, micros, from, reserved, costs, true_costs)
                }
                _ => brake(*from),
            };
            reserve(current_plan, reserved);

            match current_plan {
                Plan::Stationary(_) => true,
                Plan::Moving(_) => false,
            }
        });
    }

    fn add_new_finished(&mut self, plans: &mut HashMap<usize, Plan>, micros: &u128) {
        let new_finished = plans
            .iter_mut()
            .filter(|(id, _)| !self.finished.contains(id))
            .filter(|(_, plan)| finished(plan, micros))
            .map(|(id, _)| id)
            .collect::<Vec<_>>();

        for id in new_finished {
            self.finished.push(*id);
        }
    }
}

struct HashVec {
    waiting: HashSet<usize>,
    queue: Vec<usize>,
}

impl HashVec {
    fn new() -> HashVec {
        HashVec {
            waiting: HashSet::new(),
            queue: Vec::new(),
        }
    }

    fn contains(&self, value: &usize) -> bool {
        self.waiting.contains(value)
    }

    fn push(&mut self, value: usize) {
        self.waiting.insert(value);
        self.queue.push(value);
    }

    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&usize) -> bool,
    {
        self.queue.retain(|value| {
            let out = f(value);
            if !out {
                self.waiting.remove(value);
            }
            out
        })
    }
}

fn finished(current_plan: &Plan, micros: &u128) -> bool {
    if let Plan::Moving(ref events) = current_plan {
        if let Some(last) = events.last() {
            if *micros < last.micros {
                return false;
            }
        }
    }
    true
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
    true_costs: &HashMap<State, u64>,
) -> Plan {
    match find_path(terrain, from, reserved, costs, true_costs) {
        Some(edges) => {
            if edges.is_empty() {
                brake(*from)
            } else {
                Plan::Moving(events(micros, edges))
            }
        }
        None => brake(*from),
    }
}

fn find_path(
    terrain: &Grid<f32>,
    from: &State,
    reserved: &Grid<bool>,
    costs: &HashMap<State, u64>,
    true_costs: &HashMap<State, u64>,
) -> Option<Vec<Edge<State>>> {
    let network = SkiingNetwork { terrain, reserved };

    let from_cost = costs[from];

    network.find_best_within_steps(
        HashSet::from([*from]),
        &|_, state| {
            let Some(cost) = costs.get(state) else {
                return None;
            };
            let Some(true_cost) = true_costs.get(state) else {
                return None;
            };

            // check for forbidden tiles
            if cost != &0 // goal tile is never forbidden
                && (state.position == from.position || is_white_tile(&state.position))
            {
                return None;
            }

            Some(Score {
                cost: *true_cost,
                mode: state.mode,
            })
        },
        &|state| {
            costs
                .get(state)
                .map(|cost| *cost <= from_cost)
                .unwrap_or_default()
        },
        MAX_STEPS,
    )
}

fn is_white_tile(position: &XY<u32>) -> bool {
    position.x % 2 == position.y % 2
}

fn brake(state: State) -> Plan {
    let mode = match state.mode {
        Mode::Walking => Mode::Walking,
        Mode::Skiing { .. } => Mode::Skiing { velocity: 0 },
    };
    Plan::Stationary(State { mode, ..state })
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

#[derive(Eq)]
struct Score {
    cost: u64,
    mode: Mode,
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.mode == other.mode
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.mode, other.mode) {
            (Mode::Walking, Mode::Walking) => self.cost.cmp(&other.cost),
            (Mode::Walking, Mode::Skiing { .. }) => std::cmp::Ordering::Less,
            (Mode::Skiing { .. }, Mode::Walking) => std::cmp::Ordering::Greater,
            (Mode::Skiing { velocity: a }, Mode::Skiing { velocity: b }) => {
                self.cost.cmp(&other.cost).reverse().then(a.cmp(&b))
            }
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
